package dag

import (
	"encoding/json"
	"fmt"
)

type JNode struct {
	Id     string
	Action string
}

type JEdge struct {
	From string
	To   string
}

type JSONDAG struct {
	Nodes []JNode
	Edges []JEdge
}

func (dag *DAG) BuildFromJSON(jsonData []byte) {
	var jsonModel JSONDAG
	err := json.Unmarshal(jsonData, &jsonModel)
	if err != nil {
		fmt.Println("Error unmarshalling JSON:", err)
		return
	}

	for _, el := range jsonModel.Nodes {
		dag.AddNode(el.Id, StepFactory[el.Action])
	}
	for _, el := range jsonModel.Edges {
		dag.AddEdge(el.From, el.To)
	}
}
