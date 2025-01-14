FROM gcr.io/distroless/cc-debian12:nonroot@sha256:6970a2b2cb07c68f3e15d1b5d2ba857e53da911d5d321f48a842d6b0d26984cf

COPY --from=protoc-base:ruby /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:ruby /protobuf/include/google /opt/include/google
COPY --from=protoc-base:ruby /googleapis /googleapis

ENTRYPOINT [ "/usr/local/bin/protoc" ]
