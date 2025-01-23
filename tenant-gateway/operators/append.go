package operators

import (
	"fmt"
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/internal/pkg"

	"go.uber.org/zap"
)

type AppendSettings struct {
	Note string
}

type Append struct {
	BaseOperator
}

func (*Append) Register() error {
	return nil
}

func (*Append) Deregister() {
}

func (*Append) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	log := pkg.GetLogger()
	switch hook {
	case dag.PreProcess:
		log.Debug("PreProcess!")
	default:
		log.Debug("Unhandled hook", zap.String("node", node.ID), zap.String("hook", hook.String()))
	}
	return nil
}

func (*Append) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "Append",
	}
}

func (*Append) SetupNode(node *dag.Node) error {
	settings := AppendSettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = settings
	return nil
}

func (*Append) Process(node *dag.Node, message *dag.Message) (interface{}, error) {

	config := node.Config.(AppendSettings)

	if message.Data == nil {
		return []any{"Data is null"}, nil
	}

	data, ok := message.Data.([]any)
	if !ok {
		return nil, fmt.Errorf("(Append) input is not an array")
	}
	var appendedData []any
	appendedData = append(appendedData, data...)
	appendedData = append(appendedData, fmt.Sprintf("[%s] Append '%s'", node.ID, config.Note))

	return appendedData, nil
}
