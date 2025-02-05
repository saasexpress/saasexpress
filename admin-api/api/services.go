package api

import (
	"fmt"
	"net/http"
	"saasexpress/admin-api/db"
	"strconv"

	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

func MapAPIToDBService(apiService Service) db.Service {
	return mapAPIToDBService(apiService)
}

// mapDBToAPIService maps the DB service model to the API service model.
func mapDBToAPIService(dbService db.Service) Service {
	variants := map[string]DAGVariant{}
	for _, val := range dbService.Variants {

		//v := (db.DAGVariant)((val).(db.DAGVariant))
		//v := val.(map[])

		dag := (map[string]interface{})(val.DAG)

		variants[val.Name] = DAGVariant{
			Dag: &dag,
		}
	}

	return Service{
		Id:          &dbService.ID,
		DisplayName: &dbService.DisplayName,
		ServiceUrl:  &dbService.ServiceUrl,
		Variants:    &variants,
	}
}

// mapAPIToDBService maps the API service model to the DB service model.
func mapAPIToDBService(apiService Service) db.Service {
	variants := []db.DAGVariant{}

	if apiService.Variants != nil {

		for name, v := range *apiService.Variants {
			variant := db.DAGVariant{
				Name: name,
				DAG:  *v.Dag,
			}
			variants = append(variants, variant)

		}
	}

	svc := db.Service{
		DisplayName: *apiService.DisplayName,
		Variants:    variants,
	}
	if apiService.Id != nil {
		svc.ID = *apiService.Id
	}
	if apiService.ServiceUrl != nil {
		svc.ServiceUrl = *apiService.ServiceUrl
	}

	return svc
}

// (GET /services)
func (Server) GetServices(ctx *gin.Context, params GetServicesParams) {
	services := []db.Service{}

	db.DB.Preload("Variants").Find(&services)

	results := make([]Service, len(services))
	for i, service := range services {
		results[i] = mapDBToAPIService(service)
	}

	page := 0
	recordsPerPage := 25
	if params.RecordsPerPage != nil {
		recordsPerPage = *params.RecordsPerPage
	}
	if params.Page != nil {
		page = *params.Page
	}
	totalRecords := len(results)
	totalPages := calculateTotalPages(totalRecords, recordsPerPage)

	startIndex := page * recordsPerPage
	if startIndex > totalRecords {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "Invalid paging details"})
		return
	}
	endIndex := startIndex + recordsPerPage
	if endIndex > totalRecords {
		endIndex = totalRecords
	}
	results = results[startIndex:endIndex]

	ctx.Header("paging-total-records", strconv.Itoa(totalRecords))
	ctx.Header("paging-total-pages", strconv.Itoa(totalPages))
	ctx.Header("paging-current-page", strconv.Itoa(page))
	ctx.Header("paging-page-size", strconv.Itoa(recordsPerPage))

	ctx.JSON(http.StatusOK, results)
}

// (GET /service/{id})
func (Server) GetService(ctx *gin.Context, id string) {
	var service db.Service

	service.ID = id

	result := db.DB.Preload("Variants").First(&service)
	if result.RowsAffected == 0 {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "record not found"})
		return
	}

	dto := mapDBToAPIService(service)

	ctx.JSON(http.StatusOK, dto)
}

// (DELETE /service/{id})
func (Server) DeleteService(ctx *gin.Context, id string) {
	var service db.Service

	service.ID = id
	result := db.DB.Unscoped().Delete(&service)
	if result.RowsAffected == 0 {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "record not found"})
		return
	}

	ctx.Status(http.StatusNoContent)
}

// (PUT /services/{id})
func (Server) UpdateService(ctx *gin.Context, id string) {
	var service db.Service
	var updatedService Service

	if err := ctx.BindJSON(&updatedService); err != nil {
		ctx.AbortWithError(400, err)
		return
	}

	service.ID = id
	result := db.DB.Preload("Variants").First(&service)
	if result.RowsAffected == 0 {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "record not found"})
		return
	}

	dbService := mapAPIToDBService(updatedService)

	db.DB.Transaction(func(tx *gorm.DB) error {

		tx.Model(&service).Debug().Omit("Variants").Updates(dbService)
		if tx.Error != nil {
			return tx.Error
		}

		if updatedService.Variants != nil {
			if err := tx.Model(&service).Debug().Association("Variants").Clear(); err != nil {
				return err
			}
			if err := tx.Model(&service).Debug().Association("Variants").Append(dbService.Variants); err != nil {
				return err
			}
		}
		return nil
	})

	ctx.JSON(http.StatusOK, service)
}

// (POST /services)
func (Server) CreateService(ctx *gin.Context) {

	var newService Service

	if ctx.ContentType() == "application/yaml" {
		if err := ctx.BindYAML(&newService); err != nil {
			ctx.AbortWithError(400, err)
			return
		}
	} else {
		if err := ctx.BindJSON(&newService); err != nil {
			ctx.AbortWithError(400, err)
			return
		}
	}

	fmt.Println(newService)
	dbService := mapAPIToDBService(newService)
	if dbService.ID == "" {
		dbService.ID = db.RandStringRunes(15)
	}

	tx := db.DB.Create(&dbService)
	if tx.Error != nil {
		ctx.AbortWithError(400, tx.Error)
		return
	}

	newService.Id = &dbService.ID
	ctx.JSON(http.StatusOK, newService)
}
