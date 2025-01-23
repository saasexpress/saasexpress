package operators

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"html/template"
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/eip/channels"
	"saasexpress/tenant-gateway/internal/pkg"
	"saasexpress/tenant-gateway/schema"
	"strings"

	"go.uber.org/zap"
)

type ContentEnricherSettings struct {
	Template string
}

type ContentEnricher struct {
	BaseOperator
}

func (c *ContentEnricher) Register() error {
	validator := schema.SchemaValidator{}
	if err := validator.LoadSpec(); err != nil {
		return err
	}

	//schema := c.Spec().Schema
	jsonData := []byte(`{"name": "Bill" }`)
	var payload interface{}
	if err := json.Unmarshal([]byte(jsonData), &payload); err != nil {
		return err
	}
	if err := validator.Validate(context.Background(), payload); err != nil {
		return err
	}

	return nil
}

func (*ContentEnricher) Deregister() {
}

func (*ContentEnricher) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*ContentEnricher) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "ContentEnricher",
	}
}

func (*ContentEnricher) SetupNode(node *dag.Node) error {
	settings := ContentEnricherSettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = settings
	return nil
}

func (*ContentEnricher) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	config := node.Config.(ContentEnricherSettings)

	httpDetails, err := getHttpDetails(message)
	if err != nil {
		return nil, err
	}

	values := map[string]any{
		"in":          message.Data.(map[string]interface{}),
		"http_method": httpDetails.Request.Method,
		"http_status": httpDetails,
		"params":      httpDetails.Params,
		// pkg.MexprString("request.method", *httpDetails, ""),
	}

	log := pkg.GetLogger()

	log.Debug("Parsing template", zap.String("template", config.Template))

	tmpl, err := template.New(node.ID).Parse(strings.TrimSpace(config.Template))
	if err != nil {
		fmt.Println("Error preparing template", err)
		return nil, err
	}

	var doc bytes.Buffer
	err = tmpl.Execute(&doc, values)
	if err != nil {
		fmt.Println("Error processing template", err)
		return nil, err
	}
	log.Debug("Parsed template", zap.Any("d=", doc.String()))

	var output interface{}
	err = json.Unmarshal(doc.Bytes(), &output)
	if err != nil {
		fmt.Println("error unmarshaling json", err)
		return nil, err

	}

	return output, nil
}

func getHttpDetails(message *dag.Message) (*channels.HTTPChannel, error) {
	value, ok := message.Context.Scratchpad.GetValue("Service.ReverseProxy")
	if !ok {
		return nil, fmt.Errorf("request in details missing")
	}
	httpChannel, ok := value.(*channels.HTTPChannel)
	if !ok {
		return nil, fmt.Errorf("casting to HTTPChannel did not work")
	}
	return httpChannel, nil
}
