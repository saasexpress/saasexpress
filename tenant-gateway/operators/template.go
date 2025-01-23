package operators

import (
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/internal/pkg"
)

type TemplateSettings struct {
}

type Template struct {
	BaseOperator
}

func (*Template) Register() error {
	return nil
}

func (*Template) Deregister() {
}

func (*Template) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*Template) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "Template",
	}
}

func (*Template) SetupNode(node *dag.Node) error {
	settings := TemplateSettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = settings
	return nil
}

func (*Template) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
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
