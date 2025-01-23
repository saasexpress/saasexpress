package operators

import (
	"bytes"
	"encoding/json"
	"saasexpress/tenant-gateway/dag"
)

type JSONToBufferSettings struct {
}

type JSONToBuffer struct {
	BaseOperator
}

func (*JSONToBuffer) Register() error {
	return nil
}

func (*JSONToBuffer) Deregister() {
}

func (*JSONToBuffer) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*JSONToBuffer) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "JSONToBuffer",
	}
}

func (*JSONToBuffer) SetupNode(node *dag.Node) error {

	settings := JSONToBufferSettings{}
	node.Config = settings

	return nil
}

func (*JSONToBuffer) Process(node *dag.Node, message *dag.Message) (interface{}, error) {

	// Marshal with indentation
	indentedJSON, err := json.MarshalIndent(message.Data, "", "  ")
	if err != nil {
		return nil, err
	}

	// Write the indented JSON to a buffer
	buffer := bytes.NewBuffer(indentedJSON)

	return buffer, nil
}
