package eip

import (
	"context"
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/internal/pkg"
)

type key int

const (
	nextStep          key = 1
	responseWriterKey key = 2
)

// Serve serves
func Serve(ctx context.Context, config *pkg.Specification, dags []*dag.DAG) error {
	// upstreamServerURL, _ := url.Parse(config.UpstreamUrl)
	// serveAt := fmt.Sprintf(":%d", config.Port)

	// r := gin.Default()

	// r.POST("/gw/dag", DAGAllInOneGinHandler(config))

	// r.NoRoute(DAGGinHandler(config, dags))

	// authConfigLocation := c.String("auth-config")
	// authConfig, _ := pkg.ParseConfig(&authConfigLocation)
	// http.Handle("/metrics", promhttp.Handler())

	// http.HandleFunc("/gw/dag", LogRequest(DAGAllInOneHandler(config)))
	// http.HandleFunc("/", LogRequest(DAGHandler(config, dags)))

	// if err := http.ListenAndServe(serveAt, nil); err != nil {
	// 	log.Fatalf("Server can not start %v", err)
	// 	return err
	// }

	// serve := &http.Server{
	// 	Handler: r,
	// 	Addr:    serveAt,
	// }

	// if err := serve.ListenAndServe(); err != nil {
	// 	log.Fatalf("Server can not start %v", err)
	// 	return err
	// }

	// ginhttpin := GinHTTPInChannelService{}
	// ginhttpin.Init(ctx, config, dags)

	return nil
}
