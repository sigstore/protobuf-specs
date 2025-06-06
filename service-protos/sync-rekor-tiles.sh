#!/usr/bin/env bash

# A simple utility to copy protos from the rekor-tiles repository into protobuf specs for distribution
set -o pipefail -o errexit -o nounset

command -v gh &>/dev/null || { echo "gh not found" 1>&2; exit 1; }

latest_tag=$(gh api graphql -f query='{repository(owner: "sigstore", name: "rekor-tiles"){refs(refPrefix: "refs/tags/", last: 1){nodes{name}}}}' --jq '.data.repository.refs.nodes[].name')

if [ -z "$latest_tag" ]; then
  echo "latest tag not found"
  exit 1
fi

# clean up last sync
rm ./rekor/v2/*.proto || echo "nothing to clean up"

# mkdir just in case
mkdir -p ./rekor/v2

# copy all protos over except rekor_service.proto
echo "syncing protos with ${latest_tag}"
git clone --filter=blob:none --no-checkout --depth=1 https://github.com/sigstore/rekor-tiles.git ./rekor-tiles
cd ./rekor-tiles
git sparse-checkout set --no-cone '/api/proto/rekor/v2/*.proto' '!**/rekor_service.proto'
git fetch origin tag "$latest_tag" --no-tags
git checkout "$latest_tag"
cd ../
cp -R ./rekor-tiles/api/proto/* .
rm -rf ./rekor-tiles

# replace the go package from the service definition to the protobuf out
echo "replacing go-package"
sed -i -e 's|^option go_package.*$|option go_package = "github.com/sigstore/protobuf-specs/gen/pb-go/rekor/v2";|' ./rekor/v2/*.proto
