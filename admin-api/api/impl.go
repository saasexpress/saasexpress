package api

import (
	"net/http"
	"saasexpress/admin-api/db"

	"github.com/gin-gonic/gin"
)

// ensure that we've conformed to the `ServerInterface` with a compile-time check
var _ ServerInterface = (*Server)(nil)

type Server struct{}

func NewServer() Server {
	return Server{}
}

// (GET /tenants)
func (Server) GetTenants(ctx *gin.Context) {
	tenants := []Tenant{}

	db.DB.Find(&tenants)

	ctx.JSON(http.StatusOK, tenants)
}

// (GET /tenant/{id})
func (Server) GetTenant(ctx *gin.Context, id string) {
	var tenant db.Tenant

	tenant.ID = id
	db.DB.First(&tenant)

	ctx.JSON(http.StatusOK, tenant)
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
	// if newTenant.DisplayName != nil {
	// 	dbTenant.DisplayName = *newTenant.DisplayName
	// }
	dbTenant.ID = db.RandStringRunes(15)
	db.DB.Create(&dbTenant)

	newTenant.Id = &dbTenant.ID
	newTenant.DisplayName = dbTenant.DisplayName
	ctx.JSON(http.StatusOK, newTenant)
}
