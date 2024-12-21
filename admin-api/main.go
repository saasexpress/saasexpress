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

	// create a type that satisfies the `api.ServerInterface`, which contains an implementation of every operation from the generated code
	server := api.NewServer()

	r := gin.Default()

	api.RegisterHandlers(r, server)

	s := &http.Server{
		Handler: r,
		Addr:    "0.0.0.0:8081",
	}

	log.Println("Listing on port 8081")

	// And we serve HTTP until the world ends.
	log.Fatal(s.ListenAndServe())
}
