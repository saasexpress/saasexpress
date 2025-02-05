package pkg

import (
	"github.com/patrickmn/go-cache"
	"go.opentelemetry.io/otel/trace"
)

// Default port: `echo "saasexpress" | base64 | sed 's/[^0-9]*//g'`

type Specification struct {
	Debug             bool   `default:"false"`
	Port              int    `required:"true" default:"2243"`
	NamespaceLabel    string `required:"false"`
	NamespaceClaim    string `required:"false"`
	AdminRole         string
	VerifyToken       bool   `default:"false"`
	JwksUrl           string `required:"false"`
	UpstreamUrl       string `required:"false"`
	ResourceServerUrl string `required:"false"`
	LCache            *cache.Cache
	Tracer            trace.Tracer
}
