package dag

import (
	"sync"

	"github.com/reactivex/rxgo/v2"
)

type NodeFunction func(node *Node, input interface{}) (interface{}, error)

type JSON map[string]interface{}

type Node struct {
	ID       string
	Action   NodeFunction
	Children []*Node
	Parents  []*Node
	wg       sync.WaitGroup
	ch       chan rxgo.Item
	producer rxgo.Observable
}

type DAG struct {
	Nodes map[string]*Node
}

type Input struct {
	Item interface{}
	List []interface{}
}
