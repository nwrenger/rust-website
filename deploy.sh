#/bin/bash
set -e
cargo build -r
ssh aws-web rm -r website
scp -r target/release/website static/ aws-web:website