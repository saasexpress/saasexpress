package main

import (
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

func main() {
	var serverOptions api.GinServerOptions
	serverOptions.BaseURL = "/api"

	// create a type that satisfies the `api.ServerInterface`, which contains an implementation of every operation from the generated code
	server := api.NewServer()

	r := gin.Default()

	r.StaticFS("/ui", http.Dir("ui"))
	r.StaticFile("/", "ui/redirect.html")

	api.RegisterHandlersWithOptions(r, server, serverOptions)

	s := &http.Server{
		Handler: r,
		Addr:    "0.0.0.0:8081",
	}

	log.Println("Listing on port 8081")

	// And we serve HTTP until the world ends.
	log.Fatal(s.ListenAndServe())
}
