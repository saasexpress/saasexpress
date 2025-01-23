package operators

import (
	"bytes"
	"encoding/json"
	"saasexpress/tenant-gateway/dag"
)

type BufferToJSONSettings struct {
	Note string
}

type BufferToJSON struct {
	BaseOperator
}

func (*BufferToJSON) Register() error {
	return nil
}

func (*BufferToJSON) Deregister() {
}

func (*BufferToJSON) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*BufferToJSON) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "BufferToJSON",
	}
}

func (*BufferToJSON) SetupNode(node *dag.Node) error {

	settings := BufferToJSONSettings{}
	node.Config = settings

	return nil
}

func (*BufferToJSON) Process(node *dag.Node, message *dag.Message) (interface{}, error) {

	var output interface{}

	var buffer = message.Data.(*bytes.Buffer)

	err := json.Unmarshal(buffer.Bytes(), &output)
	if err != nil {
		return nil, err
	}
	return output, nil
}
