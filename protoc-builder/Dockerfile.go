FROM golang:1.19-bullseye AS go-builder

ADD hack/go/go.* hack/go/tools.go tools/

# the specific versions of these tools are in hack/go.mod so that Dependabot can bump them for updates
RUN cd tools && go build --trimpath google.golang.org/grpc/cmd/protoc-gen-go-grpc
RUN cd tools && go build --trimpath google.golang.org/protobuf/cmd/protoc-gen-go

FROM gcr.io/distroless/cc-debian12@sha256:b7550f0b15838de14c564337eef2b804ba593ae55d81ca855421bd52f19bb480

COPY --from=go-builder /go/tools/protoc-* /usr/local/bin/
COPY --from=protoc-base /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base /protobuf/include/google /opt/include/google
COPY --from=protoc-base /googleapis /googleapis

ENTRYPOINT ["/usr/local/bin/protoc", "--plugin=protoc-gen-go=/usr/local/bin/protoc-gen-go", "--plugin=protoc-gen-go-grpc=/usr/local/bin/protoc-gen-go-grpc"]
