package operators

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"net/http/httputil"
	"net/url"
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/eip/channels"
	"saasexpress/tenant-gateway/internal/pkg"

	"go.uber.org/zap"
)

type ReverseProxySettings struct {
	UpstreamServerURL string
	ReverseProxy      *httputil.ReverseProxy
}

type ReverseProxy struct {
	BaseOperator
}

func (*ReverseProxy) Register() error {
	return nil
}

func (*ReverseProxy) Deregister() {
}

func (*ReverseProxy) HandleHook(hook dag.HookType, node *dag.Node, message *dag.Message) error {
	return nil
}

func (*ReverseProxy) Spec() *OperatorSpec {
	return &OperatorSpec{
		Name: "ReverseProxy",
	}
}

func (*ReverseProxy) SetupNode(node *dag.Node) error {
	log := pkg.GetLogger()
	settings := ReverseProxySettings{}
	pkg.MapSettings(node.Config, &settings)
	node.Config = &settings

	upstreamServerURL, err := url.Parse(settings.UpstreamServerURL)
	if err != nil {
		log.Error("Failed to parse upstream url", zap.Error(err))
		return err
	}
	var proxy = httputil.NewSingleHostReverseProxy(upstreamServerURL)

	proxy.Transport = DebugTransport{}

	settings.ReverseProxy = proxy
	return nil
}

func (*ReverseProxy) Process(node *dag.Node, message *dag.Message) (interface{}, error) {
	return processNoRP(node, message)

	// log := pkg.GetLogger()
	// settings := node.Config.(*ReverseProxySettings)

	// value, ok := message.Context.Scratchpad.GetValue("Service.ReverseProxy")
	// if !ok {
	// 	return nil, fmt.Errorf("request in details missing")
	// }
	// httpChannel, ok := value.(*channels.HTTPChannel)
	// if !ok {
	// 	return nil, fmt.Errorf("casting to HTTPChannel did not work")
	// }

	// log.Debug("RP", zap.Any("host", httpChannel.Request.Host), zap.String("Upstream", settings.UpstreamServerURL))

	// data := []byte("{}")
	// buf := &BufferReadCloser{Buffer: bytes.NewBuffer(data)}

	// httpChannel.Request.Body = buf
	// httpChannel.Request.ContentLength = int64(buf.Buffer.Len())
	// httpChannel.Request.URL.Path = "/api/tenants"

	// settings.ReverseProxy.ServeHTTP(httpChannel.ResponseWriter, httpChannel.Request)

	// return nil, nil
}

// BufferReadCloser is a buffer that implements io.ReadCloser.
type BufferReadCloser struct {
	*bytes.Buffer
}

// Close is a no-op close method to satisfy the io.Closer interface.
func (b *BufferReadCloser) Close() error {
	// No resources to release, so just return nil.
	return nil
}

type DebugTransport struct{}

func (DebugTransport) RoundTrip(r *http.Request) (*http.Response, error) {
	log := pkg.GetLogger()
	b, err := httputil.DumpRequestOut(r, false)
	if err != nil {
		return nil, err
	}
	log.Debug(string(b))
	return http.DefaultTransport.RoundTrip(r)
}

/*
 *
**
*/
func processNoRP(node *dag.Node, message *dag.Message) (interface{}, error) {
	log := pkg.GetLogger()
	settings := node.Config.(*ReverseProxySettings)

	value, ok := message.Context.Scratchpad.GetValue("Service.ReverseProxy")
	if !ok {
		return nil, fmt.Errorf("request in details missing")
	}
	httpChannel, ok := value.(*channels.HTTPChannel)
	if !ok {
		return nil, fmt.Errorf("casting to HTTPChannel did not work")
	}

	var r = httpChannel.Request

	upstream, err := url.Parse(settings.UpstreamServerURL + httpChannel.Request.URL.Path + "?" + httpChannel.Request.URL.RawQuery)
	if err != nil {
		return nil, fmt.Errorf("failed to parse upstream url")
	}
	upstreamUrl := upstream.String()

	var buffer = message.Data.(*bytes.Buffer)

	client := &http.Client{}

	log.Debug("Calling", zap.String("method", r.Method), zap.String("upstream", upstreamUrl))
	log.Debug("Body", zap.Any("body", buffer))
	req, err := http.NewRequest(r.Method, upstreamUrl, buffer)
	if err != nil {
		log.Error("New request error", zap.String("url", upstreamUrl), zap.Error(err))
		return nil, fmt.Errorf("init request error error")
	}

	req.Header.Add("Content-Type", r.Header.Get("Content-Type"))

	resp, err := client.Do(req)
	if err != nil {
		log.Error("Fetching error", zap.String("url", upstreamUrl), zap.Error(err))
		return nil, fmt.Errorf("fetching error")
	}
	defer resp.Body.Close()

	log.Debug("", zap.String("url", upstreamUrl), zap.String("Status", resp.Status))

	if resp.StatusCode >= 300 {
		log.Error("Returned error", zap.String("url", upstreamUrl))
		return nil, fmt.Errorf("[%s] %s", node.ID, resp.Status)
	}

	httpChannel.ResponseWriter.Header().Add("Content-Type", resp.Header.Get("Content-Type"))

	httpChannel.SetStatus(resp.StatusCode)

	if resp.StatusCode == 201 || resp.StatusCode == 204 {
		respbody := []byte("{}")
		return bytes.NewBuffer(respbody), nil
	}

	respbody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Error("Read error", zap.String("url", upstreamUrl), zap.Error(err))
		return nil, fmt.Errorf("error reading response")
	}

	return bytes.NewBuffer(respbody), nil
}
