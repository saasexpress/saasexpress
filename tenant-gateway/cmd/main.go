package main

import (
	"context"
	_ "embed"
	"errors"
	"os"
	"os/signal"
	"time"

	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/eip"
	"saasexpress/tenant-gateway/internal/pkg"
	"saasexpress/tenant-gateway/operators"

	"github.com/danielgtaylor/mexpr"
	"github.com/kelseyhightower/envconfig"
	"github.com/patrickmn/go-cache"
	"go.uber.org/zap"
)

var (
	version = "dev"
	commit  = "none"
	date    = "unknown"
)

func main() {
	var log = pkg.GetLogger()

	defer func() {
		if err := recover(); err != nil {
			log.Error("panic occurred:", zap.Any("error", err))
		}
	}()

	var s pkg.Specification
	err := envconfig.Process("myapp", &s)
	if err != nil {
		log.Fatal(err.Error())
	}

	// Handle SIGINT (CTRL+C) gracefully.
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	// Set up OpenTelemetry.
	otelShutdown, err := pkg.SetupOTel(ctx, &s)
	if err != nil {
		return
	}
	// Handle shutdown properly so nothing leaks.
	defer func() {
		err = errors.Join(err, otelShutdown(context.Background()))
	}()

	// Initialize temporary cache
	s.LCache = cache.New(10*time.Minute, 10*time.Minute)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	service := &dag.MyService{}

	if err := service.Init(ctx); err != nil {
		log.Info("Initialization failed", zap.Error(err))
		return
	}
	defer func() {
		if err := service.Destroy(); err != nil {
			log.Info("Error during cleanup", zap.Error(err))
		}
	}()

	operatorService := &operators.MyService{}

	if err := operatorService.Init(ctx); err != nil {
		log.Info("[Operator] Initialization failed", zap.Error(err))
		return
	}
	defer func() {
		if err := operatorService.Destroy(); err != nil {
			log.Info("[Operator] Error during cleanup", zap.Error(err))
		}
	}()

	// Initialize DAG Operators
	operators, err := operatorService.GetRegisteredOperators()
	if err != nil {
		log.Info("[Operator] Initialization failed", zap.Error(err))
		return
	}

	dags, err := service.RegisterDAGS(&s, operators)
	if err != nil {
		log.Info("[DAGs] Initialization failed", zap.Error(err))
		return
	}

	log.Info("Started", zap.Bool("debug", s.Debug), zap.Int("port", s.Port))

	mexprTest()

	// Create a channel to capture errors from ListenAndServe
	errChan := make(chan error, 1)

	// Start server
	ginhttpin := eip.GinHTTPInChannelService{}

	go func() {
		errChan <- ginhttpin.Init(ctx, &s, dags)
	}()

	// Handle server errors asynchronously
	err = <-errChan
	if err != nil {
		log.Error("Server error", zap.Error(err))
		os.Exit(1) // Exit the program on server error
	}

}

func mexprTest() {
	var log = pkg.GetLogger()

	inputStr := "2 * 8"
	out, err := mexpr.Eval(inputStr, nil)
	if err != nil {
		log.Error(err.Pretty(inputStr))
	}
	log.Info("", zap.Float64("Answer", out.(float64)))

	inputStr = "2 * foo"
	out, err = mexpr.Eval(inputStr, map[string]interface{}{
		"foo": 2,
	})
	if err != nil {
		log.Error(err.Pretty(inputStr))
	}
	log.Info("", zap.Float64("Answer", out.(float64)))
}
