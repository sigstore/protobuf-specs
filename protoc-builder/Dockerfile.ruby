FROM gcr.io/distroless/static-debian12:nonroot@sha256:c0f429e16b13e583da7e5a6ec20dd656d325d88e6819cafe0adb0828976529dc

COPY --from=protoc-base:ruby /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:ruby /protobuf/include/google /opt/include/google
COPY --from=protoc-base:ruby /googleapis /googleapis

ENTRYPOINT [ "/usr/local/bin/protoc" ]
