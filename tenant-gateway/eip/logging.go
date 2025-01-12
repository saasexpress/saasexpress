package eip

import (
	"bytes"
	"context"
	"log"
	"net/http"
)

// LogRequest can be used as a middleware chain to log every request before proxying the request
func LogRequest(handler http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		wrapper := &ResponseWriterWrapper{
			ResponseWriter: w,
			body:           &bytes.Buffer{},
		}

		ctx := context.WithValue(r.Context(), responseWriterKey, wrapper)

		log.Printf("[FROM]\t%s %s %s\n", r.RemoteAddr, r.Method, r.URL)
		defer func() {
			log.Printf("[Logging] ON WAY BACK")
		}()
		handler(wrapper, r.WithContext(ctx))
	}
}
