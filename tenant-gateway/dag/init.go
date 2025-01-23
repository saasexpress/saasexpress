package dag

import (
	"context"
	"saasexpress/tenant-gateway/internal/pkg"
	"slices"

	"github.com/reactivex/rxgo/v2"
	"go.uber.org/zap"
)

func (dag *DAG) InitializeNodes(producerCtx context.Context, cancel context.CancelFunc, operators map[string]NodeOperator) {
	log := pkg.GetLogger()

	ctx, span := dag.Tracer.Start(dag.TracerContext, "run-dag")
	defer span.End()

	// Set so we get child spans
	dag.TracerContext = ctx

	var producers = map[string]rxgo.Observable{}

	for nid, node := range dag.Nodes {
		node.Operator = operators[node.Name]

		if node.Operator == nil {
			panic("Action does not exist" + node.Name)
		}

		node.Operator.SetupNode(node)

		producers[nid] = producer(cancel, node)
	}

	for _, node := range dag.Nodes {
		if node.HasParents() {
			setupInboundObserver(producers, node)
		}
	}

	for nid := range dag.Nodes {
		producers[nid].Connect(producerCtx)
	}

	log.Debug("initialized nodes", zap.Int("NodeCount", len(dag.Nodes)))
}

func setupInboundObserver(producers map[string]rxgo.Observable, node *Node) {
	log := pkg.GetLogger()

	observables := make([]rxgo.Observable, len(node.Parents))

	for i, parent := range node.Parents {
		observables[i] = producers[parent.ID]

		// We are waiting for all messages to come in from
		// the parents nodes.
		//
		// If just one parent, then send to node channel where
		// producerObservable is subscribed to, to proces
		//
		// If the node is not in the message BroadcastFilter
		// then do not send message to node channel.
		// Instead send to "end" as it is considered terminated
		// if node has no children, if it has children then we
		// really don't know what will happen - a child could
		// end up merging back to another node, or terminating
		// "End" has to have a flag to indicate whether it is
		// part of the response or not
		// Or require a "Response" node for joining and returning
		// "end" - so be explicit about ending in request-reply
		//
		observables[i].DoOnNext(func(item interface{}) {
			message := item.(Message)

			if message.BroadcastFilter != nil {

				log.Debug("BroadcastFilters", zap.Any("filter", message.BroadcastFilter))
				if !slices.Contains(message.BroadcastFilter, node.ID) {
					log.Debug("BroadcastFilter - Ignore", zap.String("node", node.ID))
					messageOUT := Message{
						Context: message.Context,
						Data:    nil,
					}
					node.ch <- rxgo.Of(messageOUT)
					return
				} else {
					log.Debug("BroadcastFilter - Receive", zap.String("node", node.ID))
				}
			}

			if len(node.Parents) == 1 {
				node.ch <- rxgo.Of(message)
			} else {
				scratchpad := message.Context.Scratchpad

				var m, loaded = scratchpad.LoadOrStore("combined", []interface{}{message.Data})
				if loaded {
					scratchpad.SetValue("combined", append(m.([]interface{}), message.Data))
				}
				c, _ := scratchpad.GetValue("combined")

				log.Debug("Observed", zap.String("node", node.ID))
				if len(node.Parents) == len(c.([]interface{})) {
					messageOUT := Message{
						Context: message.Context,
						Data:    c,
					}
					node.ch <- rxgo.Of(messageOUT)
				}
			}
		})
	}
}
