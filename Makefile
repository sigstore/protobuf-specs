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

PROTOC_IMAGE = protobuf-specs-build
JSONSCHEMA_IMAGE = jsonschema-specs-build

RUST_ACTION ?= run -p sigstore-protobuf-specs-codegen

# generate all language protobuf code
all: go python typescript ruby jsonschema rust

CUSER=$(shell id -u ${USER})
CGROUP=$(shell id -g ${USER})
DOCKER_RUN=docker run --platform linux/amd64 -u ${CUSER}:${CGROUP} -v ${PWD}:/defs
DOCKER_BUILD=docker build --platform linux/amd64 --build-arg uid=${CUSER}

# generate Go protobuf code
go: docker-image
	@echo "Generating go protobuf files"
	${DOCKER_RUN} ${PROTOC_IMAGE} \
		-d /defs/protos \
		-o /defs/gen/pb-go \
		-l go --go-module-prefix github.com/sigstore/protobuf-specs/gen/pb-go

python: docker-image
	@echo "Generating python protobuf files"
# we need to manually fix the PYTHONPATH due to: https://github.com/namely/docker-protoc/pull/356
	${DOCKER_RUN} -e PYTHONPATH="/opt/mypy-protobuf/" \
		--entrypoint bash ${PROTOC_IMAGE} \
		-c "cd /defs/gen/pb-python/sigstore_protobuf_specs && protoc -I/opt/include -I../../../protos/ --python_betterproto_out=. ../../../protos/*.proto"

typescript: docker-image
	@echo "Generating javascript protobuf files"
	${DOCKER_RUN} ${PROTOC_IMAGE} \
		-d /defs/protos -l typescript \
		-o /defs/gen/pb-typescript/src/__generated__ \
		--ts_opt oneof=unions,forceLong=string,env=node,exportCommonSymbols=false,outputPartialMethods=false,outputEncodeMethods=false,unrecognizedEnum=false

ruby: docker-image
	@echo "Generating ruby protobuf files"
	${DOCKER_RUN} --entrypoint bash ${PROTOC_IMAGE} \
		-c "cd /defs/gen/pb-ruby && protoc -I/opt/include -I../../protos/ --ruby_out=lib ../../protos/*.proto"

jsonschema: docker-image-jsonschema
	@echo "Generating JSON schema files"
	${DOCKER_RUN} --entrypoint sh \
		${JSONSCHEMA_IMAGE} \
		-c "cd /defs/gen/jsonschema && ./jsonschema.sh -I ../../protos -I /home/builder/googleapis/ --jsonschema_out=schemas ../../protos/*.proto"

rust: docker-image
	@echo "Generating rust protobuf files"
	${DOCKER_RUN} -e "RUST_BACKTRACE=1" \
		-e "CARGO_REGISTRY_TOKEN" \
		--entrypoint bash ${PROTOC_IMAGE} \
		-c "cd /defs/gen/pb-rust && cargo ${RUST_ACTION}"

# docker already does its own caching so we can attempt a build every time
.PHONY: docker-image
docker-image:
	@echo "Building development docker image"
	${DOCKER_BUILD} -t ${PROTOC_IMAGE} .

# to recover from a situation where a stale layer exist, just  purging the
# docker image via `make clean` is not enough. Re-building without layer
# cache is the only solution.
.PHONY: docker-image-no-cache
docker-image-no-cache:
	@echo "Building development docker image with disabled cache"
	${DOCKER_BUILD} --no-cache -t ${PROTOC_IMAGE} .

.PHONY: docker-image-jsonschema
docker-image-jsonschema:
	@echo "Building docker image for generating JSON schema files"
	${DOCKER_BUILD} -t ${JSONSCHEMA_IMAGE} -f Dockerfile.jsonschema .

# clean up generated files (not working? try sudo make clean)
clean:
	rm -rf gen/pb-go \
		gen/pb-typescript/src/__generated__ \
		gen/pb-python/sigstore_protobuf_specs/dev \
		gen/pb-python/sigstore_protobuf_specs/io \
		gen/pb-rust/target
	docker rmi -f ${PROTOC_IMAGE} ${JSONSCHEMA_IMAGE}

help:
	docker run --pull always --platform linux/amd64 -v ${PWD}:/defs ${PROTOC_IMAGE}
