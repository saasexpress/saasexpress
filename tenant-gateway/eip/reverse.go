package eip

import (
	"log"
	"net/http"
	"net/http/httputil"

	"saasexpress/tenant-gateway/internal/pkg"
)

func ReverseUpstream(reverseProxy *httputil.ReverseProxy, config *pkg.Specification) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		//r.Host = prometheusServerURL.Host
		//r.URL.Scheme = "http"
		//r.URL.Host = "localhost:5173"

		next := r.Context().Value(nextStep)

		log.Printf("NextRoute %s", r.Context().Value(nextStep))

		if next == "CreateActivity" {

		}
		log.Printf("[TO]\t%s %s %s\n", r.RemoteAddr, r.Method, r.URL)

		reverseProxy.ServeHTTP(w, r)
	}
}
