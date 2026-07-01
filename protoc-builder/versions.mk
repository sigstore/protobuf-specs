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
DEFAULT_PROTOC_VERSION=v35.1

# sha256 of release zip file: sha256sum protoc-${DEFAULT_PROTOC_VERSION#v}-linux-x86_64.zip | awk '{print "sha256:" $1 }'
DEFAULT_PROTOC_CHECKSUM=sha256:6930ebf62bd4ea607b98fff052596c6ee564b9835b4ce172c75a3f53ae9d91b7

# git commit from https://github.com/googleapis/googleapis
DEFAULT_GOOGLEAPIS_COMMIT=169dd3e4b9ca8549c7ac8a513567cc7d4dec7c83
# git commit from https://github.com/grpc-ecosystem/grpc-gateway
DEFAULT_GRPC_GATEWAY_COMMIT=3b07844f65b300da5612147be62a90dd29377def

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
