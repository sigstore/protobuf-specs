FROM golang:1.24.0-alpine@sha256:2d40d4fc278dad38be0777d5e2a88a2c6dee51b0b29c97a764fc6c6a11ca893c AS go-builder

ADD hack/go/go.* tools/

# the specific versions of these tools are in hack/go.mod so that Dependabot can bump them for updates
RUN cd tools && GOBIN=/go/tools go install tool

FROM gcr.io/distroless/static-debian12:nonroot@sha256:6ec5aa99dc335666e79dc64e4a6c8b89c33a543a1967f20d360922a80dd21f02

COPY --from=go-builder /go/tools/protoc-* /usr/local/bin/
COPY --from=protoc-base:go /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:go /protobuf/include/google /opt/include/google
COPY --from=protoc-base:go /googleapis /googleapis

ENTRYPOINT ["/usr/local/bin/protoc", "--plugin=protoc-gen-go=/usr/local/bin/protoc-gen-go", "--plugin=protoc-gen-go-grpc=/usr/local/bin/protoc-gen-go-grpc"]
