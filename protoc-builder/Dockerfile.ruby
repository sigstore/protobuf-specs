FROM gcr.io/distroless/static-debian13:nonroot@sha256:f512d819b8f109f2375e8b51d8cfd8aafe81034bc3e319740128b7d7f70d5036

COPY --from=protoc-base:ruby /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:ruby /protobuf/include/google /opt/include/google
COPY --from=protoc-base:ruby /googleapis /googleapis

ENTRYPOINT [ "/usr/local/bin/protoc" ]
