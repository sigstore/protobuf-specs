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

PROTOC_IMAGE=$(shell cat Dockerfile | grep FROM |  cut -d' ' -f2)

# generate all language protobuf code
all: go

# generate Go protobuf code
go:
	@echo "Generating go protobuf files"
	docker run --pull always --platform linux/amd64 -v ${PWD}:/defs ${PROTOC_IMAGE} -d protos -l go --go-module-prefix github.com/sigstore/protobuf-specs/gen/pb-go

# clean up generated files (not working? try sudo make clean)
clean:
	rm -rf gen

help:
	docker run --pull always --platform linux/amd64 -v ${PWD}:/defs ${PROTOC_IMAGE}
