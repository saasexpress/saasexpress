package main

import (
	"fmt"
	"log"
	"time"

	proxy "saasexpress/tenant-gateway/internal/app/query-proxy"
	"saasexpress/tenant-gateway/internal/pkg"

	"github.com/kelseyhightower/envconfig"
	"github.com/patrickmn/go-cache"
)

var (
	version = "dev"
	commit  = "none"
	date    = "unknown"
)

const ns = "namespace"

func main() {
	var s pkg.Specification
	err := envconfig.Process("myapp", &s)
	if err != nil {
		log.Fatal(err.Error())
	}
	s.LCache = cache.New(10*time.Minute, 10*time.Minute)

	format := "Debug: %v\nPort: %d\nPrometheusUrl: %s\n"
	_, err = fmt.Printf(format, s.Debug, s.Port, s.PrometheusUrl)
	if err != nil {
		log.Fatal(err.Error())
	}

	proxy.Serve(&s)

}
