# Design

## Components

### admin-api

Language: `golang`

Framework: `gin`, `air`, `gorm`, `oapi-codegen`

| Framework    | Reason                    | URL                                          |
| ------------ | ------------------------- | -------------------------------------------- |
| Gin          | HTTP Framework            | https://github.com/gin-gonic/gin             |
| Gorm         | Object-relational mapping | https://github.com/go-gorm/gorm              |
| oapi-codegen | Openapi spec to code      | https://github.com/oapi-codegen/oapi-codegen |
| gow          | dev mode watch and reload | https://github.com/mitranim/gow              |

> Maybe go back to "air"? Not sure - https://github.com/air-verse/air

Purpose: Provide a CRUD API for Tenants, Membership, Activity, Applications, Resources and Subscriptions

### tenant-gateway

Language: `golang`

Dependencies:

-     "net/http"
-     "net/http/httputil"
- "net/url"

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
