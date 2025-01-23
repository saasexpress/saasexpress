package eip

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"net/http"
	"regexp"
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/eip/channels"
	"saasexpress/tenant-gateway/internal/pkg"
	"saasexpress/tenant-gateway/operators"
	"sync"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

// GinHTTPInChannelService represents a service with a lifecycle.
type GinHTTPInChannelService struct {
	// Add fields if needed
	producerCtx context.Context
	cancel      context.CancelFunc
}

// Init initializes the service.
func (s *GinHTTPInChannelService) Init(ctx context.Context, config *pkg.Specification, dags []*dag.DAG) error {
	log := pkg.GetLogger()
	log.Debug("Initializing service...")

	serveAt := fmt.Sprintf(":%d", config.Port)

	producerCtx, cancel := context.WithCancel(context.Background())
	s.producerCtx = producerCtx
	s.cancel = cancel

	// go through the dags and register any paths for "HTTPIn"
	//
	r := gin.Default()

	r.POST("/gw/dag", DAGAllInOneGinHandler(config))

	for _, dag := range dags {
		if dag.Nodes[dag.StartID].Name == "HTTPIn" {
			settings := dag.Nodes[dag.StartID].Config.(operators.HTTPInSettings)
			valid := regexp.MustCompile(*settings.Method)
			for _, method := range []string{"GET", "PUT", "POST", "DELETE"} {
				if valid.MatchString(method) {
					var methodFunc = r.Any
					switch method {
					case "GET":
						methodFunc = r.GET
					case "PUT":
						methodFunc = r.PUT
					case "POST":
						methodFunc = r.POST
					case "DELETE":
						methodFunc = r.DELETE
					default:
						log.Warn("Unexpected method", zap.String("dag", dag.Name))
					}

					for _, route := range settings.Routes {
						log.Info("GinRoute", zap.Any("Route", route))
						methodFunc(route, DAGGinHandler(config, dag))
					}

				}
			}
		}
	}

	serve := &http.Server{
		Handler: r,
		Addr:    serveAt,
	}

	if err := serve.ListenAndServe(); err != nil {
		log.Error("Server can not start", zap.Error(err))
		return err
	}
	return nil
}

// Destroy cleans up resources used by the service.
func (s *GinHTTPInChannelService) Destroy() error {
	log := pkg.GetLogger()
	log.Debug("Destroying service...")
	s.cancel()
	// Perform cleanup tasks here
	return nil
}

func DAGGinHandler(config *pkg.Specification, aDag *dag.DAG) gin.HandlerFunc {
	return func(c *gin.Context) {
		log := pkg.GetLogger()

		log.Debug("Handler", zap.String("path", c.Request.URL.Path))
		w := c.Writer
		r := c.Request

		var httpIn channels.HTTPChannel
		httpIn.Initialize(w, r)
		httpIn.SetParams(c)
		httpIn.SetStatus = c.Status

		outBuffer, err := doTheHandling(aDag, httpIn)

		if outBuffer != nil {
			// prettyOutput(outBuffer)
			if w.Status() == http.StatusNotFound {
				c.Status(http.StatusOK)
			}
			w.Write(outBuffer.Bytes())
		} else {
			log.Error("No response", zap.Error(err))
			http.Error(w, "No response", http.StatusBadRequest)
		}
	}
}

func doTheHandling(aDag *dag.DAG, httpIn channels.HTTPChannel) (*bytes.Buffer, error) {
	log := pkg.GetLogger()

	// Using the DAG, perform the actual request
	dagContext := dag.DAGContext{
		DagModel: *aDag,
		StartID:  aDag.StartID,
		Scratchpad: &dag.ScratchpadBase{
			SafeMap: new(sync.Map),
		},
	}

	dagContext.Scratchpad.SetValue("Service.ReverseProxy", &httpIn)

	body, _ := io.ReadAll(httpIn.Request.Body)
	log.Debug("Start DAG", zap.String("DAG", aDag.Name), zap.Int("BodySize", len(body)))

	inputBuffer := bytes.NewBuffer(body)

	outBuffer, err := dagContext.RunDAG(inputBuffer)
	if err != nil {
		log.Error("Error processing DAG", zap.Error(err))
		http.Error(httpIn.ResponseWriter, "Error processing DAG", http.StatusBadRequest)
		return nil, err
	}

	log.Debug("Request Complete..")
	return outBuffer, err
}
