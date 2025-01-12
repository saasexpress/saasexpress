package dag

import (
	"context"
	"fmt"
	"log"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/reactivex/rxgo/v2"
)

func (dag *DAG) InitializeNodes(producerCtx context.Context, cancel context.CancelFunc, end chan rxgo.Item) {

	for _, node := range dag.Nodes {
		producer(cancel, node, end)
	}
	for _, node := range dag.Nodes {
		if node.HasParents() {
			setupInboundObserver(node)
		}
	}
	for _, node := range dag.Nodes {
		node.producer.Connect(producerCtx)
	}
}

func (dag *DAG) Execute(startNodeID string, input interface{}, end chan rxgo.Item) (interface{}, error) {
	startNode := dag.Nodes[startNodeID]
	totalLeafs := dag.countLeafs()

	endObservable := rxgo.FromChannel(end)

	startNode.ch <- rxgo.Of(input)

	count := 0
	var results []interface{}

	for item := range endObservable.Observe() {
		log.Println("End Observed", count)
		count++
		results = append(results, item.V)

		if count == totalLeafs {
			close(end)
			if count == 1 {
				return results[0], nil
			} else {
				return results, nil
			}
		}
	}
	return nil, fmt.Errorf("did not get a response")
}

func (dag *DAG) countLeafs() int {
	counter := 0
	for _, node := range dag.Nodes {
		if len(node.Children) == 0 {
			counter++
		}
	}
	return counter
}

func producer(_ context.CancelFunc, node *Node, end chan rxgo.Item) {
	fmt.Printf("[%s] Initialize Producer\n", node.ID)

	transfer := make(chan rxgo.Item)

	node.producer = rxgo.FromChannel(transfer, rxgo.WithPublishStrategy())

	producerObservable := rxgo.FromChannel(node.ch)

	producerObservable.DoOnNext(func(item interface{}) {

		output, err := node.Action(node, item)

		if err != nil {
			nodeState.With(prometheus.Labels{"node": node.ID, "status": "error"}).Inc()
			fmt.Println("ERROR", err)
			end <- rxgo.Of(nil)
			//cancel()
			return
		} else {
			nodeState.With(prometheus.Labels{"node": node.ID, "status": "success"}).Inc()
		}
		log.Printf("Finished action for %s [%d] ch\n", node.ID, len(node.Children))

		if len(node.Children) == 0 {
			log.Println("Sending to end channel..")
			end <- rxgo.Of(output)
		} else {
			transfer <- rxgo.Of(output)
		}
	})

	// If it is a leaf node, send it to itself so that
	// the action can run and then sent to the "end" channel
	if len(node.Children) == 0 {
		log.Println("Child setup for ending", node.ID)
		node.producer.DoOnNext(func(item interface{}) {
			log.Println("LEAF", node.ID)
			node.ch <- rxgo.Of(item)
		})
	}
}

func setupInboundObserver(node *Node) {
	observables := make([]rxgo.Observable, len(node.Parents))

	var combined []interface{}

	for i, parent := range node.Parents {
		observables[i] = parent.producer

		observables[i].DoOnNext(func(item interface{}) {
			if len(node.Parents) == 1 {
				node.ch <- rxgo.Of(item)
			} else {
				combined = append(combined, item)
				if len(node.Parents) == len(combined) {
					node.ch <- rxgo.Of(combined)
				}
			}
		})
	}
}
