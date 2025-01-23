package operators

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/internal/pkg"

	"go.uber.org/zap"
)

type HTTPRequestSettings struct {
	Url         string
	Method      string
	ContentType string
}

type HTTPRequest struct {
	BaseOperator
}

func (*HTTPRequest) Register() error {
	return nil
}

func (*HTTPRequest) Deregister() {
}

func (*HTTPRequest) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*HTTPRequest) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "HTTPRequest",
	}
}

func (*HTTPRequest) SetupNode(node *dag.Node) error {
	settings := HTTPRequestSettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = settings
	return nil
}

func (*HTTPRequest) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	log := pkg.GetLogger()

	var config = node.Config.(HTTPRequestSettings)

	var buffer = message.Data.(*bytes.Buffer)

	client := &http.Client{}

	req, err := http.NewRequest(config.Method, config.Url, buffer)
	if err != nil {
		log.Error("New request error", zap.String("url", config.Url), zap.Error(err))
		return nil, fmt.Errorf("init request error error")
	}

	if config.ContentType != "" {
		req.Header.Add("Content-Type", config.ContentType)
	}

	resp, err := client.Do(req)
	if err != nil {
		log.Error("Fetching error", zap.String("url", config.Url), zap.Error(err))
		return nil, fmt.Errorf("fetching error")
	}
	defer resp.Body.Close()

	log.Debug("", zap.String("url", config.Url), zap.String("Status", resp.Status))

	respbody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Error("Read error", zap.String("url", config.Url), zap.Error(err))
		return nil, fmt.Errorf("error reading response")
	}

	return bytes.NewBuffer(respbody), nil
}
