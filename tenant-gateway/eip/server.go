package eip

import (
	"fmt"
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"

	"saasexpress/tenant-gateway/internal/pkg"

	"github.com/prometheus/client_golang/prometheus/promhttp"
)

type key int

const (
	nextStep          key = 1
	responseWriterKey key = 2
)

// Serve serves
func Serve(config *pkg.Specification) error {
	upstreamServerURL, _ := url.Parse(config.UpstreamUrl)
	serveAt := fmt.Sprintf(":%d", config.Port)
	// authConfigLocation := c.String("auth-config")
	// authConfig, _ := pkg.ParseConfig(&authConfigLocation)
	http.Handle("/metrics", promhttp.Handler())

	http.HandleFunc("/", createHandler(upstreamServerURL, config))
	if err := http.ListenAndServe(serveAt, nil); err != nil {
		log.Fatalf("Prometheus multi tenant proxy can not start %v", err)
		return err
	}
	return nil
}

func createHandler(upstreamServerURL *url.URL, config *pkg.Specification) http.HandlerFunc {
	reverseProxy := httputil.NewSingleHostReverseProxy(upstreamServerURL)
	return LogRequest(EIP_Router(EIP_CreateActivity(ReverseUpstream(reverseProxy, config), config), config))
}
