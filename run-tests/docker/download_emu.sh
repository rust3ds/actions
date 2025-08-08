#!/bin/bash

set -euxo pipefail

TAG=$1

curl "https://api.github.com/repos/azahar-emu/azahar/releases/tags/${TAG}" |
    jq --raw-output '.assets[].browser_download_url' |
    grep -E 'azahar.*.appimage.tar.gz' |
    xargs wget -O azahar-linux.tar.gz

tar --strip-components 1 -xvf azahar-linux.tar.gz

chmod a+x azahar.AppImage
