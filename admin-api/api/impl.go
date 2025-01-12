package api

import (
	"math"
	"net/http"
	"saasexpress/admin-api/db"
	"strconv"
	"time"

	"github.com/gin-gonic/gin"
)

// ensure that we've conformed to the `ServerInterface` with a compile-time check
var _ ServerInterface = (*Server)(nil)

type Server struct{}

func NewServer() Server {
	return Server{}
}

// CreateActivity implements ServerInterface.
func (s Server) CreateActivity(ctx *gin.Context) {
	var newActivity Activity

	if ctx.ContentType() == "application/yaml" {
		if err := ctx.BindYAML(&newActivity); err != nil {
			ctx.AbortWithError(400, err)
			return
		}
	} else {
		if err := ctx.ShouldBindJSON(&newActivity); err != nil {
			ctx.AbortWithError(400, err)
			return
		}
	}

	var ts = time.Now().Format(time.RFC3339)

	var dbActivity = db.Activity{
		Message:    newActivity.Message,
		Result:     newActivity.Result,
		ActivityAt: &ts,
	}
	if newActivity.Params != nil {
		dbActivity.Params = db.JSONB(*newActivity.Params)
	}

	gorm := db.DB.Create(&dbActivity)

	if gorm.Error != nil {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "Failed to create"})
		return
	}

	params := map[string]string(dbActivity.Params)

	result := Activity{
		Id:         dbActivity.Id,
		Message:    dbActivity.Message,
		Result:     dbActivity.Result,
		Params:     &params,
		ActivityAt: dbActivity.ActivityAt,
	}
	ctx.JSON(http.StatusOK, result)
}

// DeleteActivity implements ServerInterface.
func (s Server) DeleteActivity(ctx *gin.Context, id int) {
	var activity db.Activity

	activity.Id = &id
	result := db.DB.Unscoped().Delete(&activity)
	if result.RowsAffected == 0 {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "RecordNotFound"})
		return
	}

	ctx.Status(http.StatusNoContent)
}

func calculateTotalPages(totalRecords, rowsPerPage int) int {
	// Calculate total pages and round up
	return int(math.Ceil(float64(totalRecords) / float64(rowsPerPage)))
}

// GetActivity implements ServerInterface.
func (s Server) GetActivity(ctx *gin.Context, params GetActivityParams) {
	activity := []db.Activity{}

	db.DB.Order("activity_at desc, id desc").Find(&activity)

	results := []Activity{}
	for _, element := range activity {
		params := map[string]string(db.JSONB(element.Params))

		results = append(results, Activity{
			Id:         element.Id,
			Message:    element.Message,
			Result:     element.Result,
			Params:     &params,
			ActivityAt: element.ActivityAt,
		})
	}

	totalRecords := len(results)
	totalPages := calculateTotalPages(totalRecords, *params.RecordsPerPage)

	startIndex := *params.Page * *params.RecordsPerPage
	if startIndex > totalRecords {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "Invalid paging details"})
		return
	}
	endIndex := startIndex + *params.RecordsPerPage
	if endIndex > totalRecords {
		endIndex = totalRecords
	}
	results = results[startIndex:endIndex]

	ctx.Header("paging-total-records", strconv.Itoa(totalRecords))
	ctx.Header("paging-total-pages", strconv.Itoa(totalPages))
	ctx.Header("paging-current-page", strconv.Itoa(*params.Page))
	ctx.Header("paging-page-size", strconv.Itoa(*params.RecordsPerPage))

	ctx.JSON(http.StatusOK, results)
}

// (GET /tenants)
func (Server) GetTenants(ctx *gin.Context) {
	tenants := []Tenant{}

	db.DB.Find(&tenants)

	ctx.JSON(http.StatusOK, tenants)
}

// (GET /tenant/{id})
func (Server) GetTenant(ctx *gin.Context, id string) {
	var dto Tenant
	var tenant db.Tenant

	tenant.ID = id

	result := db.DB.First(&tenant)
	if result.RowsAffected == 0 {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "RecordNotFound"})
		return
	}

	dto.Id = &tenant.ID
	dto.DisplayName = tenant.DisplayName

	ctx.JSON(http.StatusOK, dto)
}

// (DELETE /tenant/{id})
func (Server) DeleteTenant(ctx *gin.Context, id string) {
	var tenant db.Tenant

	tenant.ID = id
	result := db.DB.Unscoped().Delete(&tenant)
	if result.RowsAffected == 0 {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "RecordNotFound"})
		return
	}

	ctx.Status(http.StatusNoContent)
}

// (PUT /tenants/{id})
func (Server) UpdateTenant(ctx *gin.Context, id string) {
	var tenant db.Tenant
	var updatedTenant Tenant

	if err := ctx.BindJSON(&updatedTenant); err != nil {
		ctx.AbortWithError(400, err)
		return
	}

	tenant.ID = id
	result := db.DB.First(&tenant)
	if result.RowsAffected == 0 {
		ctx.AbortWithStatusJSON(400, gin.H{"errors": "RecordNotFound"})
		return
	}

	db.DB.Model(&tenant).Update("display_name", updatedTenant.DisplayName)

	ctx.JSON(http.StatusOK, tenant)
}

// (POST /tenants)
func (Server) CreateTenant(ctx *gin.Context) {

	var newTenant Tenant

	if ctx.ContentType() == "application/yaml" {
		if err := ctx.BindYAML(&newTenant); err != nil {
			ctx.AbortWithError(400, err)
			return
		}
	} else {
		if err := ctx.BindJSON(&newTenant); err != nil {
			ctx.AbortWithError(400, err)
			return
		}
	}

	var dbTenant db.Tenant
	if newTenant.DisplayName != nil {
		dbTenant.DisplayName = newTenant.DisplayName
	}
	dbTenant.ID = db.RandStringRunes(15)
	db.DB.Create(&dbTenant)

	newTenant.Id = &dbTenant.ID
	newTenant.DisplayName = dbTenant.DisplayName
	ctx.JSON(http.StatusOK, newTenant)
}
