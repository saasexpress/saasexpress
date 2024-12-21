package main

import (
	"embed"
	"io/fs"
	"log"
	"net/http"
	"saasexpress/admin-api/api"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type Tenant struct {
	gorm.Model
	Name string
}

//go:embed ui/*
var static embed.FS

func main() {
	var serverOptions api.GinServerOptions
	serverOptions.BaseURL = "/api"

	// create a type that satisfies the `api.ServerInterface`, which contains an implementation of every operation from the generated code
	server := api.NewServer()

	r := gin.Default()

	r.GET("/", func(c *gin.Context) {
		c.Data(http.StatusOK, "text/html", []byte(`<html><head><meta http-equiv="refresh" content="0; url=/ui/"/></head></html>`))
	})

	contentStatic, _ := fs.Sub(static, "ui")
	r.StaticFS("/ui", http.FS(contentStatic))

	api.RegisterHandlersWithOptions(r, server, serverOptions)

	s := &http.Server{
		Handler: r,
		Addr:    "0.0.0.0:8081",
	}

	log.Println("Listing on port 8081")

	// And we serve HTTP until the world ends.
	log.Fatal(s.ListenAndServe())
}
