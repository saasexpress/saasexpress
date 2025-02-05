package db

import (
	"gorm.io/datatypes"
	"gorm.io/gorm"
)

type DAGVariant struct {
	Name      string            `gorm:"uniqueIndex:variant_name_idx" json:"name"`
	DAG       datatypes.JSONMap `gorm:"type:json"`
	ServiceID string            `gorm:"uniqueIndex:variant_name_idx" json:"service_id"`
}

type Service struct {
	gorm.Model
	ID          string       `gorm:"primaryKey;<-:create"`
	DisplayName string       `gorm:"size:50"`
	ServiceUrl  string       `gorm:"size:100"`
	Variants    []DAGVariant `gorm:"constraint:OnDelete:CASCADE" json:"variants"`
}
