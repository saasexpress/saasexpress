package dag

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"saasexpress/tenant-gateway/internal/pkg"

	"go.uber.org/zap"
)

type Config struct {
	// Add fields like Port, DB Connection String, etc.
}

// Lifecycle interface defines the Init and Destroy methods.
type Lifecycle interface {
	Init() error
	Destroy() error
	RegisterDAGS() ([]DAG, error)
}

// MyService represents a service with a lifecycle.
type MyService struct {
	// Add fields if needed
	producerCtx context.Context
	cancel      context.CancelFunc
}

// Init initializes the service.
func (s *MyService) Init(ctx context.Context) error {
	log := pkg.GetLogger()
	log.Debug("Initializing service...")

	producerCtx, cancel := context.WithCancel(context.Background())
	s.producerCtx = producerCtx
	s.cancel = cancel

	// Perform initialization tasks here
	return nil
}

// Destroy cleans up resources used by the service.
func (s *MyService) Destroy() error {
	log := pkg.GetLogger()
	log.Debug("Destroying service...")
	s.cancel()
	// Perform cleanup tasks here
	return nil
}

func (s *MyService) RegisterDAGS(config *pkg.Specification, operators map[string]NodeOperator) ([]*DAG, error) {
	log := pkg.GetLogger()
	dags := []*DAG{}

	// Call admin api to get all services
	//var buffer = bytes.NewBuffer()

	url := "http://localhost:8081/api/services"

	client := &http.Client{}

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Error("Failed to read files", zap.Error(err))
		return nil, err
	}

	req.Header.Add("ContentType", "application/json")

	resp, err := client.Do(req)
	if err != nil {
		log.Error("Fetching error", zap.String("url", url), zap.Error(err))
		return nil, fmt.Errorf("fetching error")
	}
	defer resp.Body.Close()

	log.Info("", zap.String("url", url), zap.String("Status", resp.Status))

	respbody, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Error("Read error", zap.String("url", url), zap.Error(err))
		return nil, fmt.Errorf("error reading response")
	}

	var services []Service

	if err = json.Unmarshal(respbody, &services); err != nil {
		log.Error("Failed to read files", zap.Error(err))
		return nil, err
	}

	for _, service := range services {
		log.Info("Service:", zap.String("id", service.ID))

		for name, entry := range service.Variants {
			log.Info("Service Variant:", zap.String("name", name))

			if dag, err := PrepDAG(entry.Dag); err == nil {
				dag.Tracer = config.Tracer
				dag.TracerContext = nil

				dag.InitializeNodes(s.producerCtx, s.cancel, operators)

				dags = append(dags, dag)
				log.Debug("Loaded DAG", zap.String("name", dag.Name))
			} else {
				return nil, err
			}
		}
	}

	return dags, nil
}

func PrepDAG(model JSONDAG) (*DAG, error) {
	var aDag DAG

	var err = aDag.BuildFromModel(model)
	if err != nil {
		return nil, err
	}

	return &aDag, nil
}
