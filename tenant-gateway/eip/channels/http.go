package channels

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type HTTPChannel struct {
	ResponseWriter http.ResponseWriter
	Request        *http.Request
	Params         map[string]string
	SetStatus      func(status int)
}

func (c *HTTPChannel) Initialize(w http.ResponseWriter, r *http.Request) {
	c.ResponseWriter = w
	c.Request = r
}

func (chl *HTTPChannel) SetParams(c *gin.Context) {
	chl.SetStatus = c.Status
	chl.Params = map[string]string{}
	for _, param := range c.Params {
		chl.Params[param.Key] = param.Value
	}
}
