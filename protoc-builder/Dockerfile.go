FROM golang:1.24.1-alpine@sha256:43c094ad24b6ac0546c62193baeb3e6e49ce14d3250845d166c77c25f64b0386 AS go-builder

ADD hack/go/go.* tools/

# the specific versions of these tools are in hack/go.mod so that Dependabot can bump them for updates
RUN cd tools && GOBIN=/go/tools go install tool

FROM gcr.io/distroless/static-debian12:nonroot@sha256:627d6c5a23ad24e6bdff827f16c7b60e0289029b0c79e9f7ccd54ae3279fb45f

COPY --from=go-builder /go/tools/protoc-* /usr/local/bin/
COPY --from=protoc-base:go /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:go /protobuf/include/google /opt/include/google
COPY --from=protoc-base:go /googleapis /googleapis

ENTRYPOINT ["/usr/local/bin/protoc", "--plugin=protoc-gen-go=/usr/local/bin/protoc-gen-go", "--plugin=protoc-gen-go-grpc=/usr/local/bin/protoc-gen-go-grpc"]
