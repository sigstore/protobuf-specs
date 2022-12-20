#!/usr/bin/env bash
set -e

REQUIRED_PROGRAMS=("gpg" "cosign" "jq" "curl" "wget" "jar")

usage() {
  echo -e "Usage: $0 \e[4mVERSION\e[0m"
  echo -e "\e[4mVERSION\e[0m is part of the tag release/java/<version>, ex v1.0.0"
  echo -e "Requires" "${REQUIRED_PROGRAMS[@]}"
}

main() {
  # accepts exactly one arg
  if [ $# -ne 1 ];
  then
        usage "$@"
        exit 1
  fi

  RELEASE_TAG=$1

  # check is all required programs are available on system
  for i in "${REQUIRED_PROGRAMS[@]}"
  do
    command -v "$i" >/dev/null 2>&1 || { 
      echo -e "required program $i was not found" >&2
      usage "$@"
      exit 1
    }
  done

  # download release from github
  echo "Downloading release release/java/${1} from github"
  RELEASE_REPO="sigstore/protobuf-specs"
  RELEASE_URL="https://api.github.com/repos/${RELEASE_REPO}/releases/tags/release/java/${RELEASE_TAG}"
  RELEASE_INFO=$(curl -s -H "Accept: application/vnd.github+json" "${RELEASE_URL}")
  RELEASE_VERSION="$(echo "$RELEASE_INFO" | jq -r '.tag_name')"

  if [ "null" == "$RELEASE_VERSION" ]; then
    echo "ERROR: could not parse tag_name from release info"
    echo "$RELEASE_INFO"
    exit 1
  fi

  RELEASE_DIR="${RELEASE_VERSION//\//_}"

  echo "Release version is: ${RELEASE_VERSION}"

  if [ -d "$RELEASE_DIR" ]; then
    echo "Directory '$RELEASE_DIR' already exists"
    exit 1
  fi

  # query the json for all the release assets
  readarray -t ASSET_URLS < <(echo "$RELEASE_INFO" | jq -r '.assets[].browser_download_url')

  echo "Downloading release artifacts"
  for i in "${ASSET_URLS[@]}"
  do
    echo "$i"
    wget -q --directory-prefix "$RELEASE_DIR" "$i"
  done
  cd "$RELEASE_DIR"

  # cosign sign all the files
  echo "Signing with cosign"
  for file in *; do
      # skip intoto attestations, they are already signed
     if [[ "$file" == *.intoto.jsonl ]] ; then
       continue;
     fi
     COSIGN_EXPERIMENTAL=1 cosign sign-blob --yes "$file" --output-signature="$file.sig" --output-certificate="$file.pem" --bundle "$file.bundle"
  done

  # then gpg sign all the files (including sigstore files)
  # this command uses gpgs default password acceptance mechansim accept a passcode
  echo "Signing with gpg"
  for file in *; do
    gpg --batch --detach-sign --armor -o "$file.asc" "$file"
  done

  # create a maven central compatible bundle jar
  echo "Creating maven bundle"
  POM=$(ls ./*.pom)
  BUNDLE_NAME=${POM%.pom}-bundle.jar
  jar -cvf "${BUNDLE_NAME}" ./*

  echo "Upload $(realpath "$BUNDLE_NAME") to maven central (https://s01.oss.sonatype.org)"
}

main "$@"
