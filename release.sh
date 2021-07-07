#!/bin/sh
cargo release --manifest-path okapi/Cargo.toml patch --dry-run
cargo release --manifest-path rocket-okapi-codegen/Cargo.toml patch --dry-run
cargo release --manifest-path rocket-okapi/Cargo.toml patch --dry-run
