package db

import (
	"gorm.io/gorm"
)

type Activity struct {
	gorm.Model
	ActivityAt  *string `gorm:"column=activity_at" json:"activityAt,omitempty"`
	Id          *int    `json:"id,omitempty"`
	Message     *string `json:"message,omitempty"`
	Params      JSONB   `gorm:"type=text"`
	Result      *string `json:"result,omitempty"`
	FilterUKey1 *string `gorm:"index:,unique"`
	FilterKey1  *string `gorm:"index:"`
}
