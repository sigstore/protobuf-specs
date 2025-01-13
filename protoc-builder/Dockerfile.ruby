FROM gcr.io/distroless/cc-debian12@sha256:b7550f0b15838de14c564337eef2b804ba593ae55d81ca855421bd52f19bb480

COPY --from=protoc-base /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base /protobuf/include/google /opt/include/google
COPY --from=protoc-base /googleapis /googleapis

ENTRYPOINT [ "/usr/local/bin/protoc" ]
