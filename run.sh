#!/bin/bash

set -eux -o pipefail

if [[ $# -lt 1 ]]; then
    echo "Usage: run.sh 3DSX_FILE"
    exit 1
fi

trap 'docker-compose down' EXIT

rm -rf   citra/out driver/out
mkdir -p citra/out driver/out

TEST_FILE=$(realpath "$1")
export TEST_FILE

docker-compose build
docker-compose up -d citra
docker-compose run driver
