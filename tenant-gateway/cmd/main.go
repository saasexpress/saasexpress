package main

import (
	_ "embed"
	"fmt"
	"log"
	"time"

	"saasexpress/tenant-gateway/eip"
	"saasexpress/tenant-gateway/internal/pkg"

	"github.com/kelseyhightower/envconfig"
	"github.com/patrickmn/go-cache"
)

var (
	version = "dev"
	commit  = "none"
	date    = "unknown"
)

func main() {
	var s pkg.Specification
	err := envconfig.Process("myapp", &s)
	if err != nil {
		log.Fatal(err.Error())
	}

	// Initialize temporary cache
	s.LCache = cache.New(10*time.Minute, 10*time.Minute)

	// Initialize DAG Operators
	eip.InitializeOperators()

	format := "Debug: %v\nPort: %d\n"

	_, err = fmt.Printf(format, s.Debug, s.Port)
	if err != nil {
		log.Fatal(err.Error())
	}

	// Start server
	eip.Serve(&s)

}
