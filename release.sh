#!/bin/sh
cargo release --manifest-path okapi/Cargo.toml --skip-tag --skip-push $@
sleep 20
cargo release --manifest-path rocket-okapi-codegen/Cargo.toml --skip-tag --skip-push $@
sleep 20
cargo release --manifest-path rocket-okapi/Cargo.toml --skip-tag --skip-push $@

