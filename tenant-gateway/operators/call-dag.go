package operators

import (
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/internal/pkg"
)

type CallDAGSettings struct {
}

type CallDAG struct {
	BaseOperator
}

func (*CallDAG) Register() error {
	return nil
}

func (*CallDAG) Deregister() {
}

func (*CallDAG) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*CallDAG) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "CallDAG",
	}
}

func (*CallDAG) SetupNode(node *dag.Node) error {
	settings := CallDAGSettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = settings
	return nil
}

func (*CallDAG) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	log := pkg.GetLogger()
	log.Error("Not implemented yet")

	return nil, nil
}

/*
Metadata:

- SVG Logo
- Async Schema defining in/out
- Resource verification (lazy vs eager)
- Validation (i.e./ all route rule nodes exist as children)
*/
