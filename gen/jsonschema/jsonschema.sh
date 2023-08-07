#!/bin/sh

set -u
set -e

protoc --plugin=/root/go/bin/protoc-gen-jsonschema \
       --jsonschema_opt=disallow_additional_properties \
       --jsonschema_opt=enforce_oneof \
       --jsonschema_opt=enums_as_strings_only \
       --jsonschema_opt=file_extension=schema.json \
       --jsonschema_opt=json_fieldnames \
       "$@"
