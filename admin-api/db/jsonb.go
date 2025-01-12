package db

import (
	"database/sql/driver"
	"encoding/json"
	"errors"
)

// JSONB is a custom type for storing JSON data in a TEXT column.
type JSONB map[string]string

// Value method to convert the JSONB type to a JSON-encoded []byte.
func (j JSONB) Value() (driver.Value, error) {
	return json.Marshal(j)
}

// Scan method to convert a JSON-encoded []byte to a JSONB type.
func (j *JSONB) Scan(src interface{}) error {
	if src == nil {
		*j = nil
		return nil
	}

	bytes, ok := src.([]byte)
	if !ok {
		return errors.New("type assertion to []byte failed")
	}

	return json.Unmarshal(bytes, j)
}
