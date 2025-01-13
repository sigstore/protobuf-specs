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

PROTOS = $(shell find protos/ -iname "*.proto" | sed 's|^|/defs/|')

# generate all language protobuf code
all: go python typescript ruby jsonschema rust

# generate Go protobuf code
go: docker-image
	@echo "Generating go proto Docker image"
	cd protoc-builder && docker build --platform ${PLATFORM} -t ${PROTOC_GO_IMAGE} -f Dockerfile.go .
	@echo "Generating go protobuf files"
	docker run --platform ${PLATFORM} -v ${PWD}:/defs ${PROTOC_GO_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --go_opt=module=github.com/sigstore/protobuf-specs/gen/pb-go --go_out=/defs/gen/pb-go ${PROTOS}

python: docker-image
	@echo "Generating python proto Docker image"
	cd protoc-builder && docker build --platform ${PLATFORM} -t ${PROTOC_PYTHON_IMAGE} -f Dockerfile.python .
	@echo "Generating python protobuf files"
	docker run --platform ${PLATFORM} -v ${PWD}:/defs ${PROTOC_PYTHON_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --python_betterproto_opt=pydantic_dataclasses --python_betterproto_out=/defs/gen/pb-python/sigstore_protobuf_specs ${PROTOS}

typescript: docker-image
	@echo "Generating typescript proto Docker image"
	cd protoc-builder && docker build --platform ${PLATFORM} -t ${PROTOC_TYPESCRIPT_IMAGE} -f Dockerfile.typescript .
	@echo "Generating javascript protobuf files"
	docker run --platform ${PLATFORM} -v ${PWD}:/defs ${PROTOC_TYPESCRIPT_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --ts_proto_out=/defs/gen/pb-typescript/src/__generated__ --ts_proto_opt=oneof=unions,forceLong=string,env=node,exportCommonSymbols=false,outputPartialMethods=false,outputEncodeMethods=false,unrecognizedEnum=false ${PROTOS}

ruby: docker-image
	@echo "Generating ruby proto Docker image"
	cd protoc-builder && docker build --platform ${PLATFORM} -t ${PROTOC_RUBY_IMAGE} -f Dockerfile.ruby .
	@echo "Generating ruby protobuf files"
	docker run --platform ${PLATFORM} -v ${PWD}:/defs ${PROTOC_RUBY_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos --ruby_out=/defs/gen/pb-ruby/lib ${PROTOS}

jsonschema: docker-image
	@echo "Generating jsonschema proto Docker image"
	cd protoc-builder && docker build --platform ${PLATFORM} -t ${PROTOC_JSONSCHEMA_IMAGE} -f Dockerfile.jsonschema .
	@echo "Generating JSON schema files"
	mkdir -p gen/jsonschema/schemas
	docker run --platform ${PLATFORM} -v ${PWD}:/defs ${PROTOC_JSONSCHEMA_IMAGE} \
	  -I/opt/include -I/googleapis -I/defs/protos \
	  --jsonschema_out=/defs/gen/jsonschema/schemas --jsonschema_opt=disallow_additional_properties --jsonschema_opt=enforce_oneof --jsonschema_opt=enums_as_strings_only --jsonschema_opt=file_extension=schema.json --jsonschema_opt=json_fieldnames \
      ${PROTOS}

rust: docker-image
	@echo "Generating rust proto Docker image"
	cd protoc-builder && docker build -t ${PROTOC_RUST_IMAGE} -f Dockerfile.rust .
	docker run --platform ${PLATFORM} -v ${PWD}:/defs \
	  -e "RUST_BACKTRACE=1" -e "CARGO_REGISTRY_TOKEN" ${PROTOC_RUST_IMAGE} \
	  -c "cd /defs/gen/pb-rust && cargo ${RUST_ACTION}"

# docker already does its own caching so we can attempt a build every time
.PHONY: docker-image
docker-image:
	@echo "Building base docker image"
	cd protoc-builder && docker build --platform ${PLATFORM} -t ${PROTOC_IMAGE} -f Dockerfile.protoc .

# to recover from a situation where a stale layer exist, just  purging the
# docker image via `make clean` is not enough. Re-building without layer
# cache is the only solution.
.PHONY: docker-image-no-cache
docker-image-no-cache:
	@echo "Building development docker images with disabled cache"
	cd protoc-builder && docker build --no-cache -t ${PROTOC_IMAGE} -f Dockerfile.protoc .
	cd protoc-builder && docker build --no-cache -t ${PROTOC_GO_IMAGE} -f Dockerfile.go .
	cd protoc-builder && docker build --no-cache -t ${PROTOC_JSONSCHEMA_IMAGE} -f Dockerfile.jsonschema .
	cd protoc-builder && docker build --no-cache -t ${PROTOC_PYTHON_IMAGE} -f Dockerfile.python .
	cd protoc-builder && docker build --no-cache -t ${PROTOC_RUBY_IMAGE} -f Dockerfile.ruby .
	cd protoc-builder && docker build --no-cache -t ${PROTOC_RUST_IMAGE} -f Dockerfile.rust .
	cd protoc-builder && docker build --no-cache -t ${PROTOC_TYPESCRIPT_IMAGE} -f Dockerfile.typescript .

# clean up generated files (not working? try sudo make clean)
clean:
	rm -rf gen/pb-go/* \
		gen/pb-typescript/src/__generated__/* \
		gen/pb-python/sigstore_protobuf_specs/dev \
		gen/pb-python/sigstore_protobuf_specs/io \
		gen/pb-rust/target \
		gen/jsonschema/schemas
	docker rmi -f ${PROTOC_IMAGE} ${PROTOC_GO_IMAGE} ${PROTOC_JSONSCHEMA_IMAGE} ${PROTOC_PYTHON_IMAGE} ${PROTOC_RUBY_IMAGE} ${PROTOC_RUST_IMAGE} ${PROTOC_TYPESCRIPT_IMAGE}
