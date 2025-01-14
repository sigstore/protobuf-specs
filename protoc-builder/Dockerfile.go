FROM golang:1.19-bullseye AS go-builder

ADD hack/go/go.* hack/go/tools.go tools/

# the specific versions of these tools are in hack/go.mod so that Dependabot can bump them for updates
RUN cd tools && go build --trimpath google.golang.org/grpc/cmd/protoc-gen-go-grpc
RUN cd tools && go build --trimpath google.golang.org/protobuf/cmd/protoc-gen-go

FROM gcr.io/distroless/cc-debian12:nonroot@sha256:6970a2b2cb07c68f3e15d1b5d2ba857e53da911d5d321f48a842d6b0d26984cf

COPY --from=go-builder /go/tools/protoc-* /usr/local/bin/
COPY --from=protoc-base /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base /protobuf/include/google /opt/include/google
COPY --from=protoc-base /googleapis /googleapis

ENTRYPOINT ["/usr/local/bin/protoc", "--plugin=protoc-gen-go=/usr/local/bin/protoc-gen-go", "--plugin=protoc-gen-go-grpc=/usr/local/bin/protoc-gen-go-grpc"]
