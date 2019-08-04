#!/bin/bash

set -e

docker build -t liuhongchao/rust-bitcoin-prometheus:latest .

COMMIT_ID=`git rev-parse HEAD`
docker tag liuhongchao/rust-bitcoin-prometheus:latest liuhongchao/rust-bitcoin-prometheus:${COMMIT_ID}