package schema

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"saasexpress/tenant-gateway/internal/pkg"

	"github.com/getkin/kin-openapi/openapi3"
	"github.com/getkin/kin-openapi/openapi3filter"
	"github.com/getkin/kin-openapi/routers/gorillamux"
	"go.uber.org/zap"
)

type SchemaValidator struct {
	doc *openapi3.T
}

func (s *SchemaValidator) LoadSpec() error {
	log := pkg.GetLogger()

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
		log.Error("Failed to load OpenAPI spec", zap.Error(err))
		return err
	}

	err = doc.Validate(context.Background())
	if err != nil {
		log.Error("OpenAPI spec validation failed", zap.Error(err))
		return err
	}

	// If no error, the spec is valid
	log.Info("OpenAPI spec is valid")

	s.doc = doc
	return nil
}

func (s *SchemaValidator) Validate(ctx context.Context, input interface{}) error {
	log := pkg.GetLogger()

	out, err := json.Marshal(input)
	if err != nil {
		log.Error("OpenAPI spec validation failed", zap.Error(err))
		return err
	}

	body := bytes.NewBuffer(out)

	router, err := gorillamux.NewRouter(s.doc)
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
		log.Error("Request validation failed", zap.Error(err))
		return err
	} else {
		log.Info("Request validation passed!")
	}
	return nil
}
