#!/bin/bash

set -euxo pipefail

CITRA_CHANNEL=$1
CITRA_RELEASE=$2

RELEASE_API="https://api.github.com/repos/citra-emu/citra-${CITRA_CHANNEL}/releases/tags/${CITRA_CHANNEL}-${CITRA_RELEASE}"

curl "${RELEASE_API}" |
    jq --raw-output '.assets[].browser_download_url' |
    grep -E 'citra-linux-.*[.]tar.gz' |
    xargs wget -O citra-linux.tar.gz

tar --strip-components 1 -xvf citra-linux.tar.gz
