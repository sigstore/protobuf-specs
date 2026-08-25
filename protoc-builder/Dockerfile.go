FROM golang:1.26.5-alpine@sha256:99e12cfb19b753915f9b9fdc5a99f1869a24a69d3a0955832d5702e7fa68f1be AS go-builder

ADD hack/go/go.* tools/

# the specific versions of these tools are in hack/go.mod so that Dependabot can bump them for updates
RUN cd tools && GOBIN=/go/tools go install tool

FROM gcr.io/distroless/static-debian13:nonroot@sha256:963fa6c544fe5ce420f1f54fb88b6fb01479f054c8056d0f74cc2c6000df5240

COPY --from=go-builder /go/tools/protoc-* /usr/local/bin/
COPY --from=protoc-base:go /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:go /protobuf/include/google /opt/include/google
COPY --from=protoc-base:go /googleapis /googleapis

ENTRYPOINT ["/usr/local/bin/protoc", "--plugin=protoc-gen-go=/usr/local/bin/protoc-gen-go", "--plugin=protoc-gen-go-grpc=/usr/local/bin/protoc-gen-go-grpc"]
