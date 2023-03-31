#/bin/bash
set -e
cargo build -r
ssh aws-web rm -r website
scp -r target/release/website templates/ static/ Rocket.toml aws-web:website