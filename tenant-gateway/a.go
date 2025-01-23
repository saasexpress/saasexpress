package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"html/template"
	"log"
	"net/http"
	"os"

	"github.com/getkin/kin-openapi/openapi3"
	"github.com/getkin/kin-openapi/openapi3filter"
	"github.com/getkin/kin-openapi/routers/gorillamux"
	"github.com/thedevsaddam/govalidator"
)

var tpl = `{
   "actor": "{{ .Actor }}",
   "message": "{{ .Message }}"
}
`

func main() {
	fmt.Println("OK")
	tmpl, err := template.New("test").Parse(tpl)
	if err != nil {
		panic(err)
	}

	sweaters := map[string]interface{}{
		"Actor":   "Bill",
		"Message": "{action}",
	}
	_ = tmpl.Execute(os.Stdout, sweaters)

	var openAPISpec = `
openapi: 3.0.0
info:
  title: Content-Enricher
  version: 0.0.1

servers:
  - url: http://internal

paths:
  /in:
    post:
      summary: Returns a list of users.
      description: Optional extended description in CommonMark or HTML.
      requestBody:
        $ref: '#/components/requestBodies/Pet'
      responses:
        "200": # status code
          description: A JSON array of user names
          content:
            application/json:
              schema:
                type: array
                items:
                  type: string
components:
  schemas:
    Tag:
      type: object
      properties:
        id:
          type: integer
          format: int64
        name:
          type: string
      xml:
        name: Tag  
    Pet:
      type: object
      required:
        - name
      properties:
        id:
          type: integer
          format: int64
        name:
          type: string
          example: doggie
        photoUrls:
          type: array
          xml:
            name: photoUrl
            wrapped: true
          items:
            type: string
        tags:
          type: array
          xml:
            name: tag
            wrapped: true
          items:
            $ref: '#/components/schemas/Tag'
        status:
          type: string
          description: pet status in the store
          enum:
            - available
            - pending
            - sold
      xml:
        name: Pet  
  requestBodies:
    Pet:
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/Pet'
        application/xml:
          schema:
            $ref: '#/components/schemas/Pet'
      description: Pet object that needs to be added to the store
      required: true
  `

	loader := openapi3.NewLoader()
	doc, err := loader.LoadFromData([]byte(openAPISpec))
	if err != nil {
		log.Fatalf("Failed to load OpenAPI spec: %v", err)
	}

	err = doc.Validate(context.Background())
	if err != nil {
		log.Fatalf("OpenAPI spec validation failed: %v", err)
	}

	// If no error, the spec is valid
	fmt.Println("OpenAPI spec is valid")

	jsonData := []byte(`{"name": "Bill" }`)
	buffer := bytes.NewBuffer(jsonData)

	validate(doc, loader.Context, buffer)

	var payload interface{}
	if err := json.Unmarshal([]byte(jsonData), &payload); err != nil {
		panic(err)
	}

	// schemaRef, ok := doc.Components.Schemas["MySchema"]
	// if !ok {
	// 	panic("Schema 'MySchema' not found in OpenAPI document")
	// }
	// schema := schemaRef.Value

	// Validate the payload against the schema
	// 	if err := schema.Validate(context.Background(), payload, openapi3.ValidationOption{}); err != nil {
	// 		fmt.Printf("Validation failed: %s\n", err)
	// 	} else {
	// 		fmt.Println("Validation passed!")
	// 	}
	// }

	rules := govalidator.MapData{
		"username": []string{"required", "between:3,8"},
		// "email":    []string{"required", "min:4", "max:20", "email"},
	}

	messages := govalidator.MapData{
		// "username": []string{"required:আপনাকে অবশ্যই ইউজারনেম দিতে হবে", "between:ইউজারনেম অবশ্যই ৩-৮ অক্ষর হতে হবে"},
		// "phone":    []string{"digits:ফোন নাম্বার অবশ্যই ১১ নম্বারের হতে হবে"},
	}

	body := []byte(`{
    "username":"Bobf",
    "email": "acope@gmail.com"  
  }`)

	// request, err := http.NewRequest("POST", "http://internal/in", bytes.NewBuffer(body))
	// if err != nil {
	// 	panic(err)
	// }

	var pl interface{}
	if err := json.Unmarshal(body, &pl); err != nil {
		panic(err)
	}

	opts := govalidator.Options{
		Data:            pl,
		Rules:           rules,    // rules map
		Messages:        messages, // custom message map (Optional)
		RequiredDefault: false,    // all the field to be pass the rules
	}
	vv := govalidator.New(opts)
	values := vv.Validate()
	json.NewEncoder(os.Stdout).Encode(values)
}

func validate(doc *openapi3.T, ctx context.Context, body *bytes.Buffer) {
	router, err := gorillamux.NewRouter(doc)
	if err != nil {
		panic(err)
	}
	// Create a dummy HTTP request to validate
	request, err := http.NewRequest("POST", "http://internal/in", body)
	if err != nil {
		panic(err)
	}

	// Add necessary headers or body to the request if required
	request.Header.Set("Content-Type", "application/json")

	// Find the route in the OpenAPI spec that matches the request
	route, pathParams, err := router.FindRoute(request)
	if err != nil {
		panic(err)
	}

	// Validate the request against the OpenAPI route
	requestValidationInput := &openapi3filter.RequestValidationInput{
		Request:    request,
		PathParams: pathParams,
		Route:      route,
	}

	if err := openapi3filter.ValidateRequest(ctx, requestValidationInput); err != nil {
		fmt.Printf("Request validation failed: %s\n", err)
	} else {
		fmt.Println("Request validation passed!")
	}
}
