# Change log
All notable changes to this project will be documented in this file.
This project follows the [Semantic Versioning standard](https://semver.org/).

## Unreleased (2024-xx-xx)

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## Version 0.7.0 (2024-01-14)

### Added
- Added `Eq` trait for various structures.

### Changed
- `merge_paths` does correctly handle the merging of paths with double `/` in the combined path.
- Updated to Rust 2021 edition.

## Version 0.7.0-rc.1 (2021-10-02)

### Added
- Forbid unsafe code in this crate. (#36)
- Allow customization of OpenApi object.
- Allow merging of OpenApi objects.
- Added `log v0.4` as a dependency.
- Added `map!` macro for easy creation of `okapi::Map` objects.
- Re-exported `Schemars` so the same version can be used without needing to import it.

### Changed
- Change `OAuthFlows` to better represent the different flows and allowed values within them.
- Renamed `derive_json_schema` feature flag to `impl_json_schema` feature flag.

### Fixed
- Fixed casing in `SecuritySchemeData`.

## Pre version 0.7.0-rc.1 (2021-06-12)
All changes before 2021-06-12 where not documented.
This is everything before and including: 4080d574bdd7d86d3061d19bf735a14efd7cd103
