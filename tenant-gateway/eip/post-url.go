package eip

import (
	"bytes"
	"fmt"
	"io"
	"log"
	"net/http"
)

func postURL(url string, contentType string, body []byte, ch chan<- string) {

	log.Printf("PostUrl %s", url)
	resp, err := http.Post(url, contentType, bytes.NewReader(body))
	if err != nil {
		ch <- fmt.Sprintf("Error fetching %s: %v", url, err)
		return
	}
	defer resp.Body.Close()

	io.ReadAll(resp.Body)
	if err != nil {
		ch <- fmt.Sprintf("Error reading response from %s: %v", url, err)
		return
	}

	ch <- fmt.Sprintf("Fetched %s with %d bytes", url, len(body))
}
