package operators

import (
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/internal/pkg"
)

type HTTPInSettings struct {
	Routes []string
	Method *string
}

type HTTPIn struct {
	BaseOperator
}

func (*HTTPIn) Register() error {
	return nil
}

func (*HTTPIn) Deregister() {
}

func (*HTTPIn) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*HTTPIn) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "HTTPIn",
	}
}

func (*HTTPIn) SetupNode(node *dag.Node) error {
	settings := HTTPInSettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = settings
	return nil
}

func (*HTTPIn) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	return message.Data, nil
}

/*
Metadata:

- SVG Logo
- Async Schema defining in/out
- Resource verification (lazy vs eager)
- Validation (i.e./ all route rule nodes exist as children)
*/
