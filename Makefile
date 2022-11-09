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

PROTOC_IMAGE := namely/protoc-all:1.50_0

# generate Go protobuf code
proto: 
	@echo "Generating Protobuf files"
	docker run -v ${PWD}:/defs ${PROTOC_IMAGE} -i protos -f envelope.proto -l go --go-module-prefix github.com/sigstore/protobuf-specs/gen/pb-go
	docker run -v ${PWD}:/defs ${PROTOC_IMAGE} -i protos -f sigstore_common.proto -l go --go-module-prefix github.com/sigstore/protobuf-specs/gen/pb-go
	docker run -v ${PWD}:/defs ${PROTOC_IMAGE} -i protos -f sigstore_rekor.proto -l go --go-module-prefix github.com/sigstore/protobuf-specs/gen/pb-go
	docker run -v ${PWD}:/defs ${PROTOC_IMAGE} -i protos -f sigstore_bundle.proto -l go --go-module-prefix github.com/sigstore/protobuf-specs/gen/pb-go
