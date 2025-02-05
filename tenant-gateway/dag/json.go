package dag

import (
	"encoding/json"

	"gopkg.in/yaml.v3"
)

type JNode struct {
	Id     string
	Action string
	Config *map[string]interface{}
}

type JEdge struct {
	From string
	To   string
}

type JSONDAG struct {
	Name  string
	Nodes []JNode
	Edges []JEdge
}

func (dag *DAG) BuildFromJSON(jsonData []byte) error {
	var model JSONDAG
	err := json.Unmarshal(jsonData, &model)
	if err != nil {
		return err
	}
	return dag.BuildFromModel(model)
}

func (dag *DAG) BuildFromYAML(yamlData []byte) error {
	var model JSONDAG
	err := yaml.Unmarshal(yamlData, &model)
	if err != nil {
		return err
	}
	return dag.BuildFromModel(model)
}

func (dag *DAG) BuildFromModel(model JSONDAG) error {
	dag.Name = model.Name
	dag.StartID = model.Nodes[0].Id

	for _, el := range model.Nodes {
		dag.AddNode(el.Id, el.Action, el.Config)
	}
	for _, el := range model.Edges {
		if err := dag.AddEdge(el.From, el.To); err != nil {
			return err
		}
	}
	return nil
}
