package operators

import (
	"saasexpress/tenant-gateway/dag"
)

type TerminateSettings struct {
}

type Terminate struct {
	BaseOperator
}

func (*Terminate) Register() error {
	return nil
}

func (*Terminate) Deregister() {
}

func (*Terminate) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*Terminate) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "Terminate",
	}
}

func (*Terminate) SetupNode(node *dag.Node) error {

	settings := TerminateSettings{}
	node.Config = settings

	return nil
}

func (*Terminate) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	return nil, nil
}
