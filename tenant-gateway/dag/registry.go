package dag

import (
	"context"
	"embed"
	"saasexpress/tenant-gateway/internal/pkg"

	"go.uber.org/zap"
)

//go:embed core/*
var embeddedFiles embed.FS

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

	// List all files in the embedded directory
	entries, err := embeddedFiles.ReadDir("core")
	if err != nil {
		log.Error("Failed to read files", zap.Error(err))
	}

	for _, entry := range entries {
		log.Debug("File name:", zap.String("name", entry.Name()))

		// If it's a file, read its content
		if !entry.IsDir() {
			content, err := embeddedFiles.ReadFile("core/" + entry.Name())
			if err != nil {
				log.Error("Failed to read file", zap.Error(err))
			}

			if dag, err := PrepDAG(content); err == nil {
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

func PrepDAG(content []byte) (*DAG, error) {
	var aDag DAG

	var err = aDag.BuildFromYAML(content)
	if err != nil {
		return nil, err
	}

	return &aDag, nil
}
