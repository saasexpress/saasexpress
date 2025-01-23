package operators

import (
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/internal/pkg"

	"go.uber.org/zap"
)

type Rule struct {
	When string
	To   string
}

type ContentBasedRouterSettings struct {
	Rules     []Rule
	Otherwise *string
}

type ContentBasedRouter struct {
	BaseOperator
}

func (*ContentBasedRouter) Register() error {
	return nil
}

func (*ContentBasedRouter) Deregister() {
}

func (*ContentBasedRouter) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*ContentBasedRouter) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "ContentBasedRouter",
	}
}

func (*ContentBasedRouter) SetupNode(node *dag.Node) error {
	settings := ContentBasedRouterSettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = settings
	return nil
}

func (*ContentBasedRouter) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	log := pkg.GetLogger()

	setOfInputs := map[string]any{
		"method": "GET",
	}

	var broadcastFilter []string

	config := node.Config.(ContentBasedRouterSettings)
	rules := config.Rules
	// log.Debug("", zap.Any("rules", rules))
	for _, rule := range rules {
		log.Debug("Eval Rule", zap.String("rule", rule.When), zap.Any("inp", setOfInputs))
		log.Debug("Result", zap.Any("res", pkg.Mexpr(rule.When, setOfInputs)))
		if pkg.Mexpr(rule.When, setOfInputs).(bool) {
			// log.Debug("Rule Hit", zap.String("go", rule.To))
			broadcastFilter = append(broadcastFilter, rule.To)
		}
	}

	if len(broadcastFilter) == 0 && config.Otherwise != nil {
		log.Debug("Using otherwise default", zap.String("node", node.ID), zap.String("to", *config.Otherwise))
		broadcastFilter = append(broadcastFilter, *config.Otherwise)
	}

	message.BroadcastFilter = broadcastFilter

	// Terminating since there is no route to proceed with
	if len(message.BroadcastFilter) == 0 {
		log.Debug("Returning nil", zap.String("node", node.ID))
		return nil, nil
	}

	return message.Data, nil
}

/*
node config will have an array like:

rules:

	"node1": "sdk.request.method == 'POST'"

Loop through the rules - and if there is a match, then call:

	for key, val := range node.config.rules {
	  if mexpr(val, sdk) {
	    ctx.Dispatch("node1")
	  }
	}
*/
