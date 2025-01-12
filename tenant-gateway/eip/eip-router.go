package eip

import (
	"context"
	"log"
	"net/http"
	"saasexpress/tenant-gateway/internal/pkg"
)

func EIP_Router(handler http.HandlerFunc, config *pkg.Specification) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Printf("EIP_Router")
		if r.URL.Path == "/gw/dag" {
			handleDAG(w, r)
		} else if r.Method == "POST" {
			ctx := context.WithValue(r.Context(), nextStep, "CreateActivity")
			handler(w, r.WithContext(ctx))
		} else {
			handler(w, r)
		}
	}
}
