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

Purpose: Provide a runtime integration and authorization service for APIs.

| Role                         | Benefit                                                                                                                                                             | Solution                                                |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| Directed Acyclic Graph (DAG) | Provide a flexible way for defining integration tasks.                                                                                                              | Home made (?)                                           |
| AsyncAPI/Schema              | validation                                                                                                                                                          | Used for validation of input/output data between tasks. |
| Caching (temporary)          | Storing state for a period of time while waiting for work to finish.                                                                                                | github.com/patrickmn/go-cache                           |
| Expression evaluator         | Used for flexible mapping for translators                                                                                                                           | danielgtaylor/mexpr, expr-lang/expr                     |
| Observable streams           | (?) Does this replace need for a DAG?  Could use both - each Node subscribes to children.  A Join Node subscribes to its parents to wait for data from all Parents. | reactivex/rxgo/v2                                       |
| Metrics                      |                                                                                                                                                                     | Prometheus                                              |

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

### external

- `nats`
- `fluentbit`
