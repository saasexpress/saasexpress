package dag

import (
	"fmt"

	"github.com/reactivex/rxgo/v2"
)

func (dag *DAG) AddNode(id string, action NodeFunction) {
	node := &Node{ID: id, Action: action}
	if dag.Nodes == nil {
		dag.Nodes = map[string]*Node{}
	}
	node.ch = make(chan rxgo.Item)

	dag.Nodes[id] = node
}

func (dag *DAG) AddEdge(from, to string) error {
	fromNode := dag.Nodes[from]
	toNode := dag.Nodes[to]

	// Check for cycles before adding the edge
	if dag.hasCycle(fromNode, toNode) {
		return fmt.Errorf("adding edge from %s to %s would create a cycle", from, to)
	}

	fromNode.Children = append(fromNode.Children, toNode)
	toNode.Parents = append(toNode.Parents, fromNode)
	toNode.wg.Add(1)

	return nil
}

func (dag *DAG) hasCycle(start, target *Node) bool {
	visited := make(map[string]bool)
	return dag.dfsCycleCheck(start, target, visited)
}

func (dag *DAG) dfsCycleCheck(current, target *Node, visited map[string]bool) bool {
	if current.ID == target.ID {
		return true
	}
	visited[current.ID] = true
	for _, child := range current.Children {
		if !visited[child.ID] {
			if dag.dfsCycleCheck(child, target, visited) {
				return true
			}
		}
	}
	visited[current.ID] = false
	return false
}
