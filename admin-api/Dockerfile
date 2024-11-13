FROM golang:1.23-alpine

RUN apk add --no-cache make

WORKDIR /go/src/app

COPY go.mod go.sum Makefile ./
COPY . .

RUN make go-tidy go-build

EXPOSE 8080

ENV GIN_MODE=release
CMD ["bin/app"]