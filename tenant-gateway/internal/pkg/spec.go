package pkg

import (
	"github.com/patrickmn/go-cache"
)

type Specification struct {
	Debug             bool   `default:"false"`
	Port              int    `required:"true" default:"8080"`
	NamespaceLabel    string `required:"false"`
	NamespaceClaim    string `required:"false"`
	AdminRole         string
	VerifyToken       bool   `default:"false"`
	JwksUrl           string `required:"false"`
	UpstreamUrl       string `required:"true"`
	ResourceServerUrl string `required:"false"`
	LCache            *cache.Cache
}
