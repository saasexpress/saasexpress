FROM golang:1.23

WORKDIR /go/src/app

COPY cmd ./cmd
COPY dag ./dag
COPY eip ./eip
COPY operators ./operators
COPY internal ./internal
COPY pkg ./pkg
COPY go.mod Makefile ./

RUN make go-tidy go-build

EXPOSE 8080

CMD ["bin/app"]