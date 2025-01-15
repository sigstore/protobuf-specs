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

PROTOC_IMAGE = protoc-base
PROTOC_GO_IMAGE = protoc-go
PROTOC_JSONSCHEMA_IMAGE = protoc-jsonschema
PROTOC_PYTHON_IMAGE = protoc-python
PROTOC_RUBY_IMAGE = protoc-ruby
PROTOC_RUST_IMAGE = protoc-rust
PROTOC_TYPESCRIPT_IMAGE = protoc-typescript

RUST_ACTION ?= run -p sigstore-protobuf-specs-codegen

PLATFORM ?= linux/amd64
UID ?= $(shell id -u)
GID ?= $(shell id -g)
DOCKER_BUILD = docker build --platform ${PLATFORM} --build-arg UID=${UID}
DOCKER_RUN = docker run --platform ${PLATFORM} --user ${UID}:${GID}

PROTOS = $(shell find protos/ -iname "*.proto" | sed 's|^|/defs/|')

include protoc-builder/versions.mk

# generate all language protobuf code
all: go python typescript ruby jsonschema rust

# generate Go protobuf code
go: base-image-go
	@echo "Generating go proto Docker image"
	cd protoc-builder && ${DOCKER_BUILD} -t ${PROTOC_GO_IMAGE} -f Dockerfile.go .
	@echo "Generating go protobuf files"
	${DOCKER_RUN} -v ${PWD}:/defs ${PROTOC_GO_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --go_opt=module=github.com/sigstore/protobuf-specs/gen/pb-go --go_out=/defs/gen/pb-go ${PROTOS}

python: base-image-python
	@echo "Generating python proto Docker image"
	cd protoc-builder && ${DOCKER_BUILD} -t ${PROTOC_PYTHON_IMAGE} -f Dockerfile.python .
	@echo "Generating python protobuf files"
	${DOCKER_RUN} -v ${PWD}:/defs ${PROTOC_PYTHON_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --python_betterproto_opt=pydantic_dataclasses --python_betterproto_out=/defs/gen/pb-python/sigstore_protobuf_specs ${PROTOS}

typescript: base-image-typescript
	@echo "Generating typescript proto Docker image"
	cd protoc-builder && ${DOCKER_BUILD} -t ${PROTOC_TYPESCRIPT_IMAGE} -f Dockerfile.typescript .
	@echo "Generating javascript protobuf files"
	${DOCKER_RUN} -v ${PWD}:/defs ${PROTOC_TYPESCRIPT_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --ts_proto_out=/defs/gen/pb-typescript/src/__generated__ --ts_proto_opt=oneof=unions,forceLong=string,env=node,exportCommonSymbols=false,outputPartialMethods=false,outputEncodeMethods=false,unrecognizedEnum=false ${PROTOS}

ruby: base-image-ruby
	@echo "Generating ruby proto Docker image"
	cd protoc-builder && ${DOCKER_BUILD} -t ${PROTOC_RUBY_IMAGE} -f Dockerfile.ruby .
	@echo "Generating ruby protobuf files"
	${DOCKER_RUN} -v ${PWD}:/defs ${PROTOC_RUBY_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos --ruby_out=/defs/gen/pb-ruby/lib ${PROTOS}

jsonschema: base-image-jsonschema
	@echo "Generating jsonschema proto Docker image"
	cd protoc-builder && ${DOCKER_BUILD} -t ${PROTOC_JSONSCHEMA_IMAGE} -f Dockerfile.jsonschema .
	@echo "Generating JSON schema files"
	mkdir -p gen/jsonschema/schemas
	${DOCKER_RUN} -v ${PWD}:/defs ${PROTOC_JSONSCHEMA_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --jsonschema_out=/defs/gen/jsonschema/schemas --jsonschema_opt=disallow_additional_properties --jsonschema_opt=enforce_oneof --jsonschema_opt=enums_as_strings_only --jsonschema_opt=file_extension=schema.json --jsonschema_opt=json_fieldnames ${PROTOS}

rust: base-image-rust
	@echo "Generating rust proto Docker image"
	cd protoc-builder && ${DOCKER_BUILD} -t ${PROTOC_RUST_IMAGE} -f Dockerfile.rust .
	${DOCKER_RUN} -v ${PWD}:/defs \
	  -e "RUST_BACKTRACE=1" -e "CARGO_REGISTRY_TOKEN" ${PROTOC_RUST_IMAGE} \
	  -c "cd /defs/gen/pb-rust && cargo ${RUST_ACTION}"

.PHONY: base-image-go
base-image-go:
	@echo "Building base docker image for Go"
	cd protoc-builder && ${DOCKER_BUILD} ${DOCKER_CACHE} -t ${PROTOC_IMAGE}:go -f Dockerfile.protoc \
	  --build-arg PROTOC_VERSION=${GO_PROTOC_VERSION} \
	  --build-arg PROTOC_CHECKSUM=${GO_PROTOC_CHECKSUM} \
	  --build-arg GOOGLEAPIS_COMMIT=${GO_GOOGLEAPIS_COMMIT} .

.PHONY: base-image-jsonschema
base-image-jsonschema:
	@echo "Building base docker image for jsonschema"
	cd protoc-builder && ${DOCKER_BUILD} ${DOCKER_CACHE} -t ${PROTOC_IMAGE}:jsonschema -f Dockerfile.protoc \
	  --build-arg PROTOC_VERSION=${JSONSCHEMA_PROTOC_VERSION} \
	  --build-arg PROTOC_CHECKSUM=${JSONSCHEMA_PROTOC_CHECKSUM} \
	  --build-arg GOOGLEAPIS_COMMIT=${JSONSCHEMA_GOOGLEAPIS_COMMIT} .

.PHONY: base-image-python
base-image-python:
	@echo "Building base docker image for Python"
	cd protoc-builder && ${DOCKER_BUILD} ${DOCKER_CACHE} -t ${PROTOC_IMAGE}:python -f Dockerfile.protoc \
	  --build-arg PROTOC_VERSION=${PYTHON_PROTOC_VERSION} \
	  --build-arg PROTOC_CHECKSUM=${PYTHON_PROTOC_CHECKSUM} \
	  --build-arg GOOGLEAPIS_COMMIT=${PYTHON_GOOGLEAPIS_COMMIT} .

.PHONY: base-image-ruby
base-image-ruby:
	@echo "Building base docker image for Ruby"
	cd protoc-builder && ${DOCKER_BUILD} ${DOCKER_CACHE} -t ${PROTOC_IMAGE}:ruby -f Dockerfile.protoc \
	  --build-arg PROTOC_VERSION=${RUBY_PROTOC_VERSION} \
	  --build-arg PROTOC_CHECKSUM=${RUBY_PROTOC_CHECKSUM} \
	  --build-arg GOOGLEAPIS_COMMIT=${RUBY_GOOGLEAPIS_COMMIT} .

.PHONY: base-image-rust
base-image-rust:
	@echo "Building base docker image for Rust"
	cd protoc-builder && ${DOCKER_BUILD} ${DOCKER_CACHE} -t ${PROTOC_IMAGE}:rust -f Dockerfile.protoc \
	  --build-arg PROTOC_VERSION=${RUST_PROTOC_VERSION} \
	  --build-arg PROTOC_CHECKSUM=${RUST_PROTOC_CHECKSUM} \
	  --build-arg GOOGLEAPIS_COMMIT=${RUST_GOOGLEAPIS_COMMIT} .

.PHONY: base-image-typescript
base-image-typescript:
	@echo "Building base docker image for Typescript"
	cd protoc-builder && ${DOCKER_BUILD} ${DOCKER_CACHE} -t ${PROTOC_IMAGE}:typescript -f Dockerfile.protoc \
	  --build-arg PROTOC_VERSION=${TYPESCRIPT_PROTOC_VERSION} \
	  --build-arg PROTOC_CHECKSUM=${TYPESCRIPT_PROTOC_CHECKSUM} \
	  --build-arg GOOGLEAPIS_COMMIT=${TYPESCRIPT_GOOGLEAPIS_COMMIT} .

# to recover from a situation where a stale layer exist, just  purging the
# docker image via `make clean` is not enough. Re-building without layer
# cache is the only solution.
.PHONY: base-image-no-cache
base-image-no-cache:
	@echo "Building development docker images with disabled cache"
	@DOCKER_CACHE="--no-cache" make base-image-go
	@DOCKER_CACHE="--no-cache" make base-image-jsonschema
	@DOCKER_CACHE="--no-cache" make base-image-python
	@DOCKER_CACHE="--no-cache" make base-image-ruby
	@DOCKER_CACHE="--no-cache" make base-image-rust
	@DOCKER_CACHE="--no-cache" make base-image-typescript

# clean up generated files (not working? try sudo make clean)
clean:
	rm -rf gen/pb-go/* \
		gen/pb-typescript/src/__generated__/* \
		gen/pb-python/sigstore_protobuf_specs/dev \
		gen/pb-python/sigstore_protobuf_specs/io \
		gen/pb-rust/target \
		gen/jsonschema/schemas
	docker rmi -f ${PROTOC_IMAGE}:go  ${PROTOC_GO_IMAGE} \
		      ${PROTOC_IMAGE}:jsonschema ${PROTOC_JSONSCHEMA_IMAGE} \
		      ${PROTOC_IMAGE}:python ${PROTOC_PYTHON_IMAGE} \
		      ${PROTOC_IMAGE}:ruby ${PROTOC_RUBY_IMAGE} \
		      ${PROTOC_IMAGE}:rust ${PROTOC_RUST_IMAGE} \
		      ${PROTOC_IMAGE}:typescript ${PROTOC_TYPESCRIPT_IMAGE}
