# The default values for protoc version and googleapis commit will be used in the build *unless* overriden.
#
# If desired to override a language-specific protoc or googleapis import, 
# set a variable with the language name prefix followed by an underscore.
# for example:
#
#GO_PROTOC_VERSION=v29.3
#GO_PROTOC_CHECKSUM=sha256:3e866620c5be27664f3d2fa2d656b5f3e09b5152b42f1bedbf427b333e90021a
#GO_GOOGLEAPIS_COMMIT=fc2697ec5327db9073b4e0aa140248f19b15d7ef

# release tag from https://github.com/protocolbuffers/protobuf
DEFAULT_PROTOC_VERSION=v32.0

# sha256 of release zip file: sha256sum protoc-${DEFAULT_PROTOC_VERSION#v}-linux-x86_64.zip | awk '{print "sha256:" $1 }'
DEFAULT_PROTOC_CHECKSUM=sha256:7ca037bfe5e5cabd4255ccd21dd265f79eb82d3c010117994f5dc81d2140ee88

# git commit from https://github.com/googleapis/googleapis
DEFAULT_GOOGLEAPIS_COMMIT=376467058c288ad34dd7aafa892a95883e4acd0c
# git commit from https://github.com/grpc-ecosystem/grpc-gateway
DEFAULT_GRPC_GATEWAY_COMMIT=5757e1e4616289cf1ff6f2b6ec0673a43da7da54

##################################################################################
### DO NOT EDIT BELOW THIS LINE, AS THESE VALUES ARE USED IN THE CORE MAKEFILE ###
##################################################################################

LANGUAGES := GO PYTHON RUBY RUST TYPESCRIPT
COMPONENTS := PROTOC_VERSION PROTOC_CHECKSUM GOOGLEAPIS_COMMIT GRPC_GATEWAY_COMMIT

# This is creating each possible variable permutation, e.g.
# GO_PROTOC_VERSION, etc
$(foreach lang,$(LANGUAGES),\
    $(foreach component,$(COMPONENTS),\
        $(eval $(lang)_$(component) ?= $$(DEFAULT_$(component)))))
