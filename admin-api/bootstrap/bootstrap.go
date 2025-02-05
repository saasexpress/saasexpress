package bootstrap

import (
	"embed"
	"encoding/json"
	"saasexpress/admin-api/api"
	"saasexpress/admin-api/db"
	"saasexpress/admin-api/shared"

	"go.uber.org/zap"
	"gopkg.in/yaml.v2"
)

type JNode struct {
	Id     string                  `json:"id"`
	Action string                  `json:"action"`
	Config *map[string]interface{} `json:"config"`
}

type JEdge struct {
	From string `json:"from"`
	To   string `json:"to"`
}

type JSONDAG struct {
	Name  string  `json:"name"`
	Nodes []JNode `json:"nodes"`
	Edges []JEdge `json:"edges"`
}

// type Data struct {
// 	name  string
// 	nodes []Entry `yaml:"nodes"`
// 	edges []Entry `yaml:"edges"`
// }

type Entry interface{}

//go:embed core/*
var embeddedFiles embed.FS

func Bootstrap(server api.Server) error {
	log := shared.GetLogger()

	log.Info("Bootstrapping services..")

	displayName := "Tenant Management"
	id := "cci.tenant-management"
	serviceUrl := "http://localhost:8081"
	service := api.Service{
		Id:          &id,
		DisplayName: &displayName,
		ServiceUrl:  &serviceUrl,
	}
	service.Variants = &map[string]api.DAGVariant{}

	// List all files in the embedded directory
	entries, err := embeddedFiles.ReadDir("core")
	if err != nil {
		log.Error("Failed to read files", zap.Error(err))
	}

	for _, entry := range entries {
		log.Info("File name:", zap.String("name", entry.Name()))

		if !entry.IsDir() {
			yamlData, err := embeddedFiles.ReadFile("core/" + entry.Name())
			if err != nil {
				log.Error("Failed to read file", zap.Error(err))
			}

			var d = JSONDAG{}
			err = yaml.Unmarshal(yamlData, &d)
			if err != nil {
				log.Error("Failed to unmarshal", zap.Error(err))
				return err
			}

			var variant = api.DAGVariant{Dag: &map[string]interface{}{
				"name":  d.Name,
				"nodes": d.Nodes,
				"edges": d.Edges,
			}}

			(*service.Variants)[entry.Name()] = variant
		}
	}

	bytes, _ := json.MarshalIndent(service, "", "  ")
	log.Info(string(bytes))

	dbService := api.MapAPIToDBService(service)

	tx := db.DB.Create(&dbService)
	if tx.Error != nil {
		log.Error("Failed to create service", zap.Error(tx.Error))
		return tx.Error
	}

	return nil
}
