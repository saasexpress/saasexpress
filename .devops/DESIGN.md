# Design

## Components

### admin-api

Language: `golang`

Framework: `gin`, `air`

- https://github.com/gin-gonic/gin
- https://github.com/air-verse/air


### tenant-gateway

Language: `golang`

Dependencies:

- 	"net/http"
- 	"net/http/httputil"
-	"net/url"

### tenant-ui

Language: `react`

Framework: `react-app` and `chakra`

```
npx create-next-app@latest
```

### testsuite

Framework: `cypress`

### website

Framework: `docusaurus`

```
npx create-docusaurus@latest website classic --typescript
```