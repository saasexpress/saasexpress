# Design

## Components

### admin-api

Language: `golang`

Framework: `gin`, `air`, `gorm`, `oapi-codegen`

- https://github.com/gin-gonic/gin
- https://github.com/air-verse/air

Purpose: Provide a CRUD API for Tenants, Membership, Activity, Applications, Resources and Subscriptions

### tenant-gateway

Language: `golang`

Dependencies:

- 	"net/http"
- 	"net/http/httputil"
-	"net/url"

Purpose: Provide a runtime authorization service for APIs.

### tenant-ui

Language: `javascript`

Framework: `react` and `chakra`

```
npx create-next-app@latest
```

Purpose: Provide a frontend for managing Tenants

### testsuite

Framework: `cypress`

### website

Framework: `docusaurus`

```
npx create-docusaurus@latest website classic --typescript
```

Purpose: Documentation for how to configure saasexpress with a new API

