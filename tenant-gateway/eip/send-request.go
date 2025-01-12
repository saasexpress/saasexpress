package eip

import (
	"io/ioutil"
	"net/http"
)

func sendRequest(url string, originalRequest *http.Request) ([]byte, error) {
	client := &http.Client{}
	req, err := http.NewRequest(originalRequest.Method, url, originalRequest.Body)
	if err != nil {
		return nil, err
	}
	req.Header = originalRequest.Header
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	return ioutil.ReadAll(resp.Body)
}
