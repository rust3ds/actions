#!/bin/bash

set -euxo pipefail

TAG=$1

curl "https://api.github.com/repos/azahar-emu/azahar/releases/tags/${TAG}" |
    jq --raw-output '.assets[].browser_download_url' |
    grep -E 'azahar.AppImage' |
    xargs wget -O azahar.AppImage

chmod a+x azahar.AppImage
