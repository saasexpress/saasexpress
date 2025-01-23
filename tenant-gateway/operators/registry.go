package operators

import (
	"context"
	"saasexpress/tenant-gateway/dag"
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
	GetRegisteredOperators() (map[string]OperatorInterface, error)
}

// MyService represents a service with a lifecycle.
type MyService struct {
	// Add fields if needed
}

// Init initializes the service.
func (s *MyService) Init(ctx context.Context) error {
	log := pkg.GetLogger()
	log.Debug("Initializing service...")
	// Perform initialization tasks here
	return nil
}

// Destroy cleans up resources used by the service.
func (s *MyService) Destroy() error {
	log := pkg.GetLogger()
	log.Debug("Destroying service...")
	// Perform cleanup tasks here
	return nil
}

func (s *MyService) GetRegisteredOperators() (map[string]dag.NodeOperator, error) {
	log := pkg.GetLogger()
	operators := map[string]dag.NodeOperator{}

	allOperators := []OperatorInterface{
		&Append{},
		&BufferToJSON{},
		&CallDAG{},
		&ContentBasedRouter{},
		&ContentEnricher{},
		&HTTPIn{},
		&HTTPRequest{},
		&JoinStrings{},
		&JSONToBuffer{},
		&NoOperation{},
		&ReverseProxy{},
		&Template{},
		&Terminate{},
	}

	for _, operator := range allOperators {
		log.Info("Register", zap.String("name", operator.Spec().Name))

		err := operator.Register()
		if err != nil {
			panic(err)
		}
		defer func() {
			operator.Deregister()
		}()

		// meta := operator.(*BaseOperator)
		// meta.Name = operator.Spec().Name

		operators[operator.Spec().Name] = operator
	}
	return operators, nil
}
