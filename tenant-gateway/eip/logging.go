package eip

import (
	"bytes"
	"context"
	"net/http"
	"saasexpress/tenant-gateway/internal/pkg"

	"go.uber.org/zap"
)

// LogRequest can be used as a middleware chain to log every request before proxying the request
func LogRequest(handler http.HandlerFunc) http.HandlerFunc {
	log := pkg.GetLogger()

	return func(w http.ResponseWriter, r *http.Request) {
		wrapper := &ResponseWriterWrapper{
			ResponseWriter: w,
			body:           &bytes.Buffer{},
		}

		ctx := context.WithValue(r.Context(), responseWriterKey, wrapper)

		log.Debug("-->", zap.String("RemoteIP", r.RemoteAddr), zap.String("Method", r.Method), zap.String("URL", r.URL.String()))

		defer func() {
			log.Debug("<--", zap.String("RemoteIP", r.RemoteAddr), zap.String("Method", r.Method), zap.String("URL", r.URL.String()))
		}()

		handler(wrapper, r.WithContext(ctx))
	}
}
