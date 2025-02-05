package dag

import (
	"context"
	"fmt"
	"saasexpress/tenant-gateway/schema"
	"sync"

	"github.com/reactivex/rxgo/v2"
	"go.opentelemetry.io/otel/trace"
)

type HookType int

const (
	PreProcess HookType = iota
	PostProcess
)

var hookName = map[HookType]string{
	PreProcess:  "PreProcess",
	PostProcess: "PostProcess",
}

func (ss HookType) String() string {
	return hookName[ss]
}

type NodeOperator interface {
	Register() error
	Deregister()
	HandleHook(hook HookType, node *Node, message *Message) error
	Process(node *Node, message *Message) (interface{}, error)
	SetupNode(node *Node) error
}

type JSON map[string]interface{}

type Service struct {
	ID          string
	DisplayName string
	ServiceUrl  string
	Variants    map[string]ServiceVariant
}

type ServiceVariant struct {
	Dag JSONDAG
}

type Node struct {
	ID              string
	Name            string
	Operator        NodeOperator
	Config          any
	SchemaValidator schema.SchemaValidator

	Children []*Node
	Parents  []*Node

	ch chan rxgo.Item
}

type DAG struct {
	Name          string
	StartID       string
	Nodes         map[string]*Node
	Tracer        trace.Tracer
	TracerContext context.Context
}

type DAGContext struct {
	Label      string
	DagModel   DAG
	StartID    string
	End        chan rxgo.Item
	Scratchpad Scratchpad
}

type Scratchpad interface {
	GetValue(key string) (any, bool)
	SetValue(key string, value any)
	LoadOrStore(key string, value any) (any, bool)
}

type ScratchpadBase struct {
	sync.Mutex
	SafeMap *sync.Map
}

type Input struct {
	Item interface{}
	List []interface{}
}

type Message struct {
	Context         *DAGContext
	Data            interface{}
	BroadcastFilter []string
}

func (dag *DAG) CountLeafs() int {
	counter := 0
	for _, node := range dag.Nodes {
		if len(node.Children) == 0 {
			counter++
		}
	}
	return counter
}

func (sp *ScratchpadBase) GetValue(key string) (any, bool) {
	sp.Lock()

	val, ok := sp.SafeMap.Load(key)
	if !ok {
		fmt.Println("dag.types.Scratchpad - Value not found")
	}
	sp.Unlock()
	return val, ok
}

func (sp *ScratchpadBase) LoadOrStore(key string, value any) (any, bool) {

	//fmt.Println("dag.types.Scratchpad - Setting " + key + " = ")
	sp.Lock()
	val, loaded := sp.SafeMap.LoadOrStore(key, value)
	sp.Unlock()

	return val, loaded
}

func (sp *ScratchpadBase) SetValue(key string, value any) {

	//fmt.Println("dag.types.Scratchpad - Setting " + key + " = ")
	sp.Lock()
	sp.SafeMap.Store(key, value)
	sp.Unlock()
}

func (d *DAGContext) Push(key string, value any) {
	d.Scratchpad.SetValue(key, value)
}
