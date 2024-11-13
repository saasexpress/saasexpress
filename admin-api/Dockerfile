FROM golang:1.23

WORKDIR /go/src/app

COPY go.mod go.sum Makefile ./
COPY main.go ./

RUN make go-tidy go-build

ENV GIN_MODE=release
CMD ["bin/app"]