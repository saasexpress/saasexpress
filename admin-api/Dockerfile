FROM golang:1.23-alpine AS build

RUN apk add --no-cache make build-base

WORKDIR /go/src/app

COPY go.mod go.sum Makefile ./
COPY . .

# RUN go mod download
# RUN go vet -v
# RUN go test -v

RUN CGO_ENABLED=1 make go-tidy go-build

FROM golang:1.23-alpine

#Not working due to CGO_ENABLED=1 - "/app: no such file or directory"
#FROM gcr.io/distroless/base:latest

WORKDIR /

COPY --from=build /go/src/app/bin/app /
COPY --from=build /go/src/app/ui /ui
ENV GIN_MODE=release
CMD ["/app"]