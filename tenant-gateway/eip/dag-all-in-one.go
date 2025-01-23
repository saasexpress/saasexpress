package eip

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"net/http"
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/eip/channels"
	"saasexpress/tenant-gateway/internal/pkg"
	"saasexpress/tenant-gateway/operators"
	"sync"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

type AllInOne struct {
	Input  interface{}
	Route  *string
	Method *string
	Nodes  []dag.JNode
}

func DAGAllInOneGinHandler(config *pkg.Specification) gin.HandlerFunc {
	return func(c *gin.Context) {
		log := pkg.GetLogger()

		log.Info("Handler", zap.String("path", c.Request.URL.Path))
		w := c.Writer
		r := c.Request

		aDag, inputBuffer, allInOne, err := loadDAGAndGetInput(r)
		if err != nil {
			http.Error(w, "Invalid JSON", http.StatusBadRequest)
		}

		// create a new context
		producerCtx, cancel := context.WithCancel(context.Background())
		defer cancel()

		operatorService := &operators.MyService{}
		operators, err := operatorService.GetRegisteredOperators()
		if err != nil {
			http.Error(w, "Invalid JSON", http.StatusBadRequest)
			return
		}

		aDag.Tracer = config.Tracer
		aDag.TracerContext = nil

		aDag.InitializeNodes(producerCtx, cancel, operators)

		// Using the DAG, perform the actual request
		dagContext := dag.DAGContext{
			DagModel: *aDag,
			StartID:  allInOne.Nodes[0].Id,
			Scratchpad: &dag.ScratchpadBase{
				SafeMap: new(sync.Map),
			},
		}

		var httpIn channels.HTTPChannel
		httpIn.Initialize(w, r)
		httpIn.SetStatus = c.Status
		if allInOne.Method != nil {
			// override values if they are set in the "all-in-one" call
			log.Info("Override", zap.String("method", *allInOne.Method), zap.String("route", *allInOne.Route))
			r.Method = *allInOne.Method
			r.URL.Path = *allInOne.Route
		}

		dagContext.Scratchpad.SetValue("Service.ReverseProxy", &httpIn)

		outBuffer, err := dagContext.RunDAG(inputBuffer)
		if err != nil {
			log.Error("Error processing DAG", zap.Error(err))
			http.Error(httpIn.ResponseWriter, "Error processing DAG", http.StatusBadRequest)
			return
		}

		if outBuffer != nil {
			// prettyOutput(outBuffer)
			if w.Status() == http.StatusNotFound {
				c.Status(http.StatusOK)
			}
			w.Write(outBuffer.Bytes())
		} else {
			log.Error("No response", zap.Error(err))
			http.Error(w, "No response", http.StatusBadRequest)
		}
	}
}

func loadDAGAndGetInput(r *http.Request) (*dag.DAG, *bytes.Buffer, *AllInOne, error) {
	log := pkg.GetLogger()

	body, _ := io.ReadAll(r.Body)
	log.Info("Start DAG", zap.Int("BodySize", len(body)))

	var aDag dag.DAG

	var err = aDag.BuildFromJSON(body)
	if err != nil {
		log.Error("Invalid DAG", zap.Error(err))
		return nil, nil, nil, err
	}

	var start AllInOne
	json.Unmarshal(body, &start)

	inputBuffer := new(bytes.Buffer)
	json.NewEncoder(inputBuffer).Encode(start.Input)

	return &aDag, inputBuffer, &start, nil
}

/*
func DAGAllInOneHandler(config *pkg.Specification) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {

		log := pkg.GetLogger()

		aDag, inputBuffer, allInOne, err := loadDAGAndGetInput(r)
		if err != nil {
			http.Error(w, "Invalid JSON", http.StatusBadRequest)
		}

		// create a new context
		producerCtx, cancel := context.WithCancel(context.Background())
		defer cancel()

		operatorService := &operators.MyService{}
		operators, err := operatorService.GetRegisteredOperators()
		if err != nil {
			http.Error(w, "Invalid JSON", http.StatusBadRequest)
			return
		}

		aDag.Tracer = config.Tracer
		aDag.TracerContext = nil

		aDag.InitializeNodes(producerCtx, cancel, operators)

		// Using the DAG, perform the actual request
		dagContext := dag.DAGContext{
			DagModel: *aDag,
			StartID:  allInOne.Nodes[0].Id,
			Scratchpad: &dag.ScratchpadBase{
				SafeMap: new(sync.Map),
			},
		}

		var httpIn channels.HTTPChannel
		httpIn.Initialize(w, r)
		r.Method = *allInOne.Method
		r.URL.Path = *allInOne.Route

		dagContext.Scratchpad.SetValue("Service.ReverseProxy", &httpIn)

		outBuffer, err := dagContext.RunDAG(inputBuffer)
		if err != nil {
			log.Error("Error processing DAG", zap.Error(err))
			http.Error(w, "Error processing DAG", http.StatusBadRequest)
			return
		}

		log.Debug("Request Complete..")

		if outBuffer != nil {
			prettyOutput(outBuffer)
			w.Write(outBuffer.Bytes())
		} else {
			log.Error("No response", zap.Error(err))
			http.Error(w, "No response", http.StatusBadRequest)
		}
	}
}

func prettyOutput(outBuffer *bytes.Buffer) {
	log := pkg.GetLogger()

	var m any

	err := json.Unmarshal(outBuffer.Bytes(), &m)
	if err != nil {
		log.Warn("Failed to unmarshall response", zap.Error(err))
		return
	}

	buf, _ := json.MarshalIndent(m, "", "  ")

	log.Info(string(buf))
}
*/
