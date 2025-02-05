package db

import (
	"log"

	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

var DB *gorm.DB

func init() {
	log.Println("Using sqlite in-memory database")

	// file::memory:?cache=shared
	db, err := gorm.Open(sqlite.Open("./sqlite.db"), &gorm.Config{})
	if err != nil {
		panic("failed to connect database")
	}

	// Migrate the schema
	db.AutoMigrate(&Tenant{})
	db.AutoMigrate(&Activity{})
	db.AutoMigrate(&Service{})
	db.AutoMigrate(&DAGVariant{})

	DB = db
}
