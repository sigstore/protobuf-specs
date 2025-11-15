FROM gcr.io/distroless/static-debian13:nonroot@sha256:fedcacdbe5eb1b1ec2ab5f2e7bf2eed246eb371505c43b3c2682ff0c3870e7c3

COPY --from=protoc-base:ruby /protobuf/bin/protoc /usr/local/bin/
COPY --from=protoc-base:ruby /protobuf/include/google /opt/include/google
COPY --from=protoc-base:ruby /googleapis /googleapis

ENTRYPOINT [ "/usr/local/bin/protoc" ]
