package eip

import (
	"bytes"
	"log"
	"net/http"
)

// ResponseWriterWrapper wraps http.ResponseWriter to capture status and body.
type ResponseWriterWrapper struct {
	http.ResponseWriter
	status int
	body   *bytes.Buffer
}

func (rw *ResponseWriterWrapper) WriteHeader(code int) {
	log.Printf("Codee = %d", code)

	rw.status = code
	rw.ResponseWriter.WriteHeader(code)
}

func (rw *ResponseWriterWrapper) Write(b []byte) (int, error) {
	rw.body.Write(b)
	return rw.ResponseWriter.Write(b)
}
