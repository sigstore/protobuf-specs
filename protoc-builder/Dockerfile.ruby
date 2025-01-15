FROM gcr.io/distroless/static-debian12:nonroot@sha256:6ec5aa99dc335666e79dc64e4a6c8b89c33a543a1967f20d360922a80dd21f02

COPY --from=protoc-base:ruby /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:ruby /protobuf/include/google /opt/include/google
COPY --from=protoc-base:ruby /googleapis /googleapis

ENTRYPOINT [ "/usr/local/bin/protoc" ]
