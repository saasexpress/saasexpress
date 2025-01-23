package operators

import (
	"saasexpress/tenant-gateway/dag"
)

type NoOperationSettings struct {
}

type NoOperation struct {
	BaseOperator
}

func (*NoOperation) Register() error {
	return nil
}

func (*NoOperation) Deregister() {
}

func (*NoOperation) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*NoOperation) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "NoOp",
	}
}

func (*NoOperation) SetupNode(node *dag.Node) error {

	settings := NoOperationSettings{}
	node.Config = settings

	return nil
}

func (*NoOperation) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	return message.Data, nil
}
