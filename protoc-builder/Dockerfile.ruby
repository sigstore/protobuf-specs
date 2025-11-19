FROM gcr.io/distroless/static-debian13:nonroot@sha256:423ba16a9ec162509175cb6904f703d3c8a5a3a58cff9b0b4fb2684bb74162c5

COPY --from=protoc-base:ruby /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:ruby /protobuf/include/google /opt/include/google
COPY --from=protoc-base:ruby /googleapis /googleapis

ENTRYPOINT [ "/usr/local/bin/protoc" ]
