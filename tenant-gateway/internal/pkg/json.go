package pkg

import (
	"encoding/json"

	"go.uber.org/zap"
)

func MapSettings(in any, out interface{}) {
	log := GetLogger()

	buf, _ := json.MarshalIndent(in, "", "  ")

	err := json.Unmarshal(buf, &out)
	if err != nil {
		log.Warn("Failed to unmarshall response", zap.Error(err))
		return
	}
}
