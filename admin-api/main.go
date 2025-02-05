package main

import (
	"embed"
	"io/fs"
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"
	"saasexpress/admin-api/api"
	"saasexpress/admin-api/bootstrap"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/kelseyhightower/envconfig"
	"gorm.io/gorm"
)

type Tenant struct {
	gorm.Model
	Name string
}

type Specification struct {
	UIProxyURL string `envconfig:"UI_PROXY_URL"`
	GWProxyURL string `envconfig:"GW_PROXY_URL"`
}

//go:embed api.yaml
var openapi embed.FS

//go:embed ui/*
var static embed.FS

func proxySetup(proxyUrl string, basePath string) gin.HandlerFunc {

	var theproxy = func(c *gin.Context) {
		remote, err := url.Parse(proxyUrl)
		if err != nil {
			panic(err)
		}

		log.Println("Proxying to", strings.Join([]string{basePath, c.Param("proxyPath")}, ""))
		proxy := httputil.NewSingleHostReverseProxy(remote)
		proxy.Director = func(req *http.Request) {
			req.Header = c.Request.Header
			req.Host = remote.Host
			req.URL.Scheme = remote.Scheme
			req.URL.Host = remote.Host
			req.URL.Path = strings.Join([]string{basePath, c.Param("proxyPath")}, "")
		}

		proxy.ServeHTTP(c.Writer, c.Request)
	}

	return theproxy
}

func main() {
	var s Specification
	err := envconfig.Process("myapp", &s)
	if err != nil {
		log.Fatal(err.Error())
	}

	var serverOptions api.GinServerOptions
	serverOptions.BaseURL = "/api"

	// create a type that satisfies the `api.ServerInterface`, which contains an implementation of every operation from the generated code
	server := api.NewServer()

	// bootstrap the database
	bootstrap.Bootstrap(server)

	r := gin.Default()

	r.GET("/", func(c *gin.Context) {
		c.Data(http.StatusOK, "text/html", []byte(`<html><head><meta http-equiv="refresh" content="0; url=/ui/"/></head></html>`))
	})

	r.StaticFileFS("/api/openapi.yaml", "api.yaml", http.FS(openapi))

	if s.UIProxyURL != "" {
		log.Println("Proxying UI to", s.UIProxyURL)
		r.Any("/ui/*proxyPath", proxySetup(s.UIProxyURL, "/ui"))
	} else {
		contentStatic, _ := fs.Sub(static, "ui")
		r.StaticFS("/ui", http.FS(contentStatic))
	}

	if s.GWProxyURL != "" {
		log.Println("Proxying GW to", s.GWProxyURL)
		r.Any("/gw/*proxyPath", proxySetup(s.GWProxyURL, "/gw"))
	}

	// Catch-all route for SPA
	r.NoRoute(func(c *gin.Context) {
		indexhtml, _ := static.ReadFile("ui/index.html")
		c.Data(http.StatusOK, "text/html", indexhtml)
	})

	api.RegisterHandlersWithOptions(r, server, serverOptions)

	serve := &http.Server{
		Handler: r,
		Addr:    "0.0.0.0:8081",
	}

	log.Println("Listing on port 8081")

	// And we serve HTTP until the world ends.
	log.Fatal(serve.ListenAndServe())
}
