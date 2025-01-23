package operators

import (
	"fmt"
	"saasexpress/tenant-gateway/dag"
	"slices"
)

type JoinStringsSettings struct {
}

type JoinStrings struct {
	BaseOperator
}

func (*JoinStrings) Register() error {
	return nil
}

func (*JoinStrings) Deregister() {
}

func (*JoinStrings) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*JoinStrings) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "JoinStrings",
	}
}

func (*JoinStrings) SetupNode(node *dag.Node) error {

	settings := JoinStringsSettings{}
	node.Config = settings

	return nil
}

func (*JoinStrings) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	data, ok := message.Data.([]any)
	if !ok {
		return nil, fmt.Errorf("input is not an array %s", message.Data)
	}
	var aggregatedData []string

	for _, item := range data[0].([]any) {
		aggregatedData = append(aggregatedData, item.(string))
	}
	for _, item := range data[1].([]any) {
		aggregatedData = append(aggregatedData, item.(string))
	}
	slices.Sort(aggregatedData)

	aggregatedData = append(aggregatedData, fmt.Sprintf("[%s] JoinStrings", node.ID))

	return aggregatedData, nil
	// return nil, fmt.Errorf("Oops")
}
