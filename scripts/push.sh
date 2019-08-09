COMMIT_ID=`git rev-parse HEAD`
docker push liuhongchao/rust-bitcoin-prometheus:latest
docker push liuhongchao/rust-bitcoin-prometheus:${COMMIT_ID}