#!/bin/bash

set -euxo pipefail

TAG=$1

curl "https://api.github.com/repos/PabloMK7/citra/releases/tags/${TAG}" |
    jq --raw-output '.assets[].browser_download_url' |
    grep -E 'citra-linux-.*[.]tar.gz' |
    xargs wget -O citra-linux.tar.gz

tar --strip-components 1 -xvf citra-linux.tar.gz
