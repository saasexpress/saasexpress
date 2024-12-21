FROM golang:1.23-alpine as build

RUN apk add --no-cache make build-base

WORKDIR /go/src/app

COPY go.mod go.sum Makefile ./
COPY . .

ENV CGO_ENABLED=1
RUN make go-tidy go-build

FROM golang:1.23-alpine

WORKDIR /app

EXPOSE 8080

COPY --from=build /go/src/app/bin/app /app/app
ENV GIN_MODE=release
CMD ["/app/app"]