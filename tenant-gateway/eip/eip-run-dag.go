package eip

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"saasexpress/tenant-gateway/dag"

	"github.com/reactivex/rxgo/v2"
)

type Start struct {
	Input interface{}
	Nodes []dag.JNode
}

func handleDAG(w http.ResponseWriter, r *http.Request) {

	observable := rxgo.Just("Hello, World!")()
	ch := observable.Observe()
	item := <-ch
	fmt.Println(item.V)

	body, _ := io.ReadAll(r.Body)
	log.Printf("DAG = %s", body)

	var dag dag.DAG

	dag.BuildFromJSON(body)

	var start Start

	json.Unmarshal(body, &start)

	log.Printf("Start with %s", start.Nodes[0].Id)

	producerCtx, cancel := context.WithCancel(context.Background())
	defer cancel()

	end := make(chan rxgo.Item)
	dag.InitializeNodes(producerCtx, cancel, end)

	output, err := dag.Execute(start.Nodes[0].Id, start.Input, end)
	if err != nil {
		log.Fatalf("Error executing DAG: %v", err)
	}

	if output == nil {
		http.Error(w, fmt.Sprintf("Error processing DAG"), http.StatusInternalServerError)
		return
	}

	outputStr, _ := json.MarshalIndent(output, "", "  ")
	fmt.Println("Final Output:", string(outputStr))

	w.Write([]byte(outputStr))

}
