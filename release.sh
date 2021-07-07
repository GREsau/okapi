#!/bin/sh
cargo release --manifest-path okapi/Cargo.toml patch --dry-run
sleep 60
cargo release --manifest-path rocket-okapi-codegen/Cargo.toml patch --dry-run
sleep 60
cargo release --manifest-path rocket-okapi/Cargo.toml patch --dry-run
