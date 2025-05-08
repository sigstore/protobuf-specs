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
DEFAULT_PROTOC_VERSION=v30.2

# sha256 of release zip file: sha256sum protoc-${DEFAULT_PROTOC_VERSION#v}-linux-x86_64.zip | awk '{print "sha256:" $1 }'
DEFAULT_PROTOC_CHECKSUM=sha256:327e9397c6fb3ea2a542513a3221334c6f76f7aa524a7d2561142b67b312a01f

# git commit from https://github.com/googleapis/googleapis
DEFAULT_GOOGLEAPIS_COMMIT=cc6c360ec4509ef0288d5e2c85bd6ec1a3b1de83

##################################################################################
### DO NOT EDIT BELOW THIS LINE, AS THESE VALUES ARE USED IN THE CORE MAKEFILE ###
##################################################################################

LANGUAGES := GO JSONSCHEMA PYTHON RUBY RUST TYPESCRIPT
COMPONENTS := PROTOC_VERSION PROTOC_CHECKSUM GOOGLEAPIS_COMMIT

# This is creating each possible variable permutation, e.g.
# GO_PROTOC_VERSION, JSONSCHEMA_PROTOC_VERSION, etc
$(foreach lang,$(LANGUAGES),\
    $(foreach component,$(COMPONENTS),\
        $(eval $(lang)_$(component) ?= $$(DEFAULT_$(component)))))
