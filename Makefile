#
# Copyright 2022 The Sigstore Authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

PROTOC_IMAGE=protobuf-specs-build

# generate all language protobuf code
all: go python

# generate Go protobuf code
go: docker-image
	@echo "Generating go protobuf files"
	docker run --platform linux/amd64 -v ${PWD}:/defs ${PROTOC_IMAGE} -d protos -l go --go-module-prefix github.com/sigstore/protobuf-specs/gen/pb-go

python: docker-image
	@echo "Generating python protobuf files"
# we need to manually fix the PYTHONPATH due to: https://github.com/namely/docker-protoc/pull/356
	docker run --platform linux/amd64 -v ${PWD}:/defs -e PYTHONPATH="/opt/mypy-protobuf/" --entrypoint make ${PROTOC_IMAGE} generate-python

# we should NEVER invoke this target manually, this target gets built inside docker via the `python` target
generate-python:
	cd ./gen/pb-python/sigstore_protobuf_specs && \
	protoc -I/opt/include -I../../../protos/ --python_betterproto_out=. ../../../protos/*

# docker already does its own caching so we can attempt a build every time
.PHONY: docker-image
docker-image:
	@echo "Building development docker image"
	docker build -t ${PROTOC_IMAGE} .

# clean up generated files (not working? try sudo make clean)
clean:
	rm -rf gen/pb-go \
		gen/pb-python/sigstore_protobuf_specs/dev \
		gen/pb-python/sigstore_protobuf_specs/io
	docker rmi -f ${PROTOC_IMAGE}

help:
	docker run --pull always --platform linux/amd64 -v ${PWD}:/defs ${PROTOC_IMAGE}
