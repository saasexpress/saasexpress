package eip

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"saasexpress/tenant-gateway/internal/pkg"
)

type Params map[string]string

type CreateResponse struct {
	Id string `json:id,omitempty`
}

type Activity struct {
	ActivityAt string `json:"activityAt,omitempty"`
	Message    string `json:"message,omitempty"`
	Params     Params `json:"params,omitempty"`
	Result     string `json:"result,omitempty"`
}

func EIP_CreateActivity(handler http.HandlerFunc, config *pkg.Specification) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {

		log.Printf("EIP_CreateActivity")
		defer func() {
			next := r.Context().Value(nextStep)

			log.Printf("EIP_CreateActivity ON WAY BACK %s", next)
			if next == "CreateActivity" {
				if wrapper, ok := r.Context().Value(responseWriterKey).(*ResponseWriterWrapper); ok {
					// Example: logging the captured status and body
					log.Printf("Status = %d", wrapper.status)
					fmt.Printf("Captured status in anotherMiddleware: %d\n", wrapper.status)
					fmt.Printf("Captured body in anotherMiddleware: %s\n", wrapper.body.String())

					var respData CreateResponse
					err := json.Unmarshal(wrapper.body.Bytes(), &respData)
					if err != nil {
						fmt.Println("Error marshaling JSON:", err)
						return
					}

					if wrapper.status == 200 {
						log.Printf("Sending Activity!")
						ch := make(chan string)
						url := config.UpstreamUrl + "/api/activity"
						activity := Activity{
							Message: "{actor} {action} {resource} ({id})",
							Result:  "success",
							Params: Params{
								"actor":    "Joe Smith",
								"action":   "Created",
								"resource": "Tenant",
								"id":       string(respData.Id),
							}}
						jsonData, err := json.Marshal(activity)
						if err != nil {
							fmt.Println("Error marshaling JSON:", err)
							return
						}
						log.Printf("JSON = %s", jsonData)
						go postURL(url, "application/json", jsonData, ch)
						fmt.Println(<-ch)
					}
				}
			} else {
				log.Printf("EIP_CreateActivity SKIP")
			}
		}()
		handler(w, r)

	}
}
