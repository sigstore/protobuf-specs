# The default values for protoc version and googleapis commit will be used in the build *unless* overriden.
#
# If desired to override a language-specific protoc or googleapis import, 
# set a variable with the language name prefix followed by an underscore.
# for example:
#
GO_PROTOC_VERSION=v29.3
GO_PROTOC_CHECKSUM=sha256:3e866620c5be27664f3d2fa2d656b5f3e09b5152b42f1bedbf427b333e90021a
GO_GOOGLEAPIS_COMMIT=fc2697ec5327db9073b4e0aa140248f19b15d7ef
PYTHON_PROTOC_VERSION=v29.3
PYTHON_PROTOC_CHECKSUM=sha256:3e866620c5be27664f3d2fa2d656b5f3e09b5152b42f1bedbf427b333e90021a
PYTHON_GOOGLEAPIS_COMMIT=fc2697ec5327db9073b4e0aa140248f19b15d7ef
RUBY_PROTOC_VERSION=v29.3
RUBY_PROTOC_CHECKSUM=sha256:3e866620c5be27664f3d2fa2d656b5f3e09b5152b42f1bedbf427b333e90021a
RUBY_GOOGLEAPIS_COMMIT=fc2697ec5327db9073b4e0aa140248f19b15d7ef
RUST_PROTOC_VERSION=v29.3
RUST_PROTOC_CHECKSUM=sha256:3e866620c5be27664f3d2fa2d656b5f3e09b5152b42f1bedbf427b333e90021a
RUST_GOOGLEAPIS_COMMIT=fc2697ec5327db9073b4e0aa140248f19b15d7ef

# TODO: add dependabot-like feature to check for release updates
# release tag from https://github.com/protocolbuffers/protobuf
DEFAULT_PROTOC_VERSION=v21.6

# TODO: add dependabot-like feature to check for release updates
# sha256 of release zip file: sha256sum protoc-${DEFAULT_PROTOC_VERSION#v}-linux-x86_64.zip | awk '{print "sha256:" $1 }'
DEFAULT_PROTOC_CHECKSUM=sha256:6a9fc36363a2d05d73fc363a46cd57d849068d33305db39f77daac8ba073e818

# TODO: add dependabot-like feature to check for release updates
# git commit from https://github.com/googleapis/googleapis
DEFAULT_GOOGLEAPIS_COMMIT=6ef9eaea379fc1cc0355e06a5a20b594543ee693

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
