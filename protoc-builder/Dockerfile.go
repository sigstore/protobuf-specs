FROM golang:1.26.1-alpine@sha256:2389ebfa5b7f43eeafbd6be0c3700cc46690ef842ad962f6c5bd6be49ed82039 AS go-builder

ADD hack/go/go.* tools/

# the specific versions of these tools are in hack/go.mod so that Dependabot can bump them for updates
RUN cd tools && GOBIN=/go/tools go install tool

FROM gcr.io/distroless/static-debian13:nonroot@sha256:e3f945647ffb95b5839c07038d64f9811adf17308b9121d8a2b87b6a22a80a39

COPY --from=go-builder /go/tools/protoc-* /usr/local/bin/
COPY --from=protoc-base:go /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:go /protobuf/include/google /opt/include/google
COPY --from=protoc-base:go /googleapis /googleapis

ENTRYPOINT ["/usr/local/bin/protoc", "--plugin=protoc-gen-go=/usr/local/bin/protoc-gen-go", "--plugin=protoc-gen-go-grpc=/usr/local/bin/protoc-gen-go-grpc"]
