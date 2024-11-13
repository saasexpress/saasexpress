FROM golang:1.23

WORKDIR /go/src/app

COPY go.mod go.sum Makefile ./
COPY . .

RUN make go-tidy go-generate go-build

EXPOSE 8080

ENV GIN_MODE=release
CMD ["bin/app"]