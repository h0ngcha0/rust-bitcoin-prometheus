#!/bin/bash

set -e

SCRIPT_DIR=`dirname $0`
WORK_DIR=${SCRIPT_DIR}/..

pushd $WORK_DIR
docker build -t liuhongchao/rust-bitcoin-prometheus:latest .
popd

COMMIT_ID=`git rev-parse HEAD`
docker tag liuhongchao/rust-bitcoin-prometheus:latest liuhongchao/rust-bitcoin-prometheus:${COMMIT_ID}