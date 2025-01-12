package eip

import (
	"fmt"
	"net/http"
	"saasexpress/tenant-gateway/internal/pkg"
	"sync"
)

func EIP_Split(handler http.HandlerFunc, config *pkg.Specification) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Define the URLs for the two backend services.
		url1 := "http://localhost:8081/service1"
		url2 := "http://localhost:8082/service2"

		// Use a WaitGroup to wait for both requests to complete.
		var wg sync.WaitGroup
		wg.Add(2)

		var response1, response2 []byte
		var err1, err2 error

		// Send the first request in a goroutine.
		go func() {
			defer wg.Done()
			response1, err1 = sendRequest(url1, r)
		}()

		// Send the second request in a goroutine.
		go func() {
			defer wg.Done()
			response2, err2 = sendRequest(url2, r)
		}()

		// Wait for both requests to complete.
		wg.Wait()

		// Check for errors.
		if err1 != nil {
			http.Error(w, fmt.Sprintf("Error from service1: %v", err1), http.StatusInternalServerError)
			return
		}
		if err2 != nil {
			http.Error(w, fmt.Sprintf("Error from service2: %v", err2), http.StatusInternalServerError)
			return
		}

		// Aggregate the responses.
		aggregatedResponse := fmt.Sprintf("Response from service1: %s\nResponse from service2: %s\n", response1, response2)

		// Send the aggregated response back to the client.
		w.Write([]byte(aggregatedResponse))
	}
}
