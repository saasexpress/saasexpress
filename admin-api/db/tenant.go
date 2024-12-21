package db

import (
	"gorm.io/gorm"
)

type Tenant struct {
	gorm.Model
	ID          string  `gorm:"primaryKey;<-:create"`
	DisplayName *string `gorm:"column=display_name"`
}
