# Change log
All notable changes to this project will be documented in this file.
This project follows the [Semantic Versioning standard](https://semver.org/).

## Version 0.8.0-alpha-1 (2021-xx-xx)

### Added
- Added feature flag `rapidoc` to enable [RapiDoc](https://mrin9.github.io/RapiDoc/) documentation
viewer. (Based on #33)
- Added RapiDoc v9.0.0
- Forbid unsafe code in this crate. (#36)
- Retrieve OpenApi object after generating. (#28)
- Create `mount_endpoints_and_merged_docs` marco in order to streamline code structure for
bigger projects. (#30)
- Added new example for structuring bigger projects.
- Allowed changing path where OpenApi file is hosted.
- Added `openapi_routes` and `openapi_spec` macros to allow lower level access to the generation
of the routes and specification respectively.
- Added `openapi_get_routes`, `openapi_get_routes_spec` and `openapi_get_spec` to get a combination
of `Vec<rocket::Route>` and/or `okapi::openapi3::OpenApi`.
- Optionally allows the setting of `OpenApiSettings` when generating the OpenApi objects and Routes.

### Changed
- Swagger UI is now only available under the feature `swagger`.
- Updated Swagger UI to v3.52.0
- `UrlObject` has been moved from `swagger_ui::UrlObject` to `settings::UrlObject`.

### Deprecated

### Removed
- Removed unused and unneeded files from Swagger UI to decrease file size.
- `routes_with_openapi` has be removed and replaced with `openapi_routes`.

### Fixed

### Security

## Pre version 0.8.0-alpha-1 (2021-06-12)
All changes before 2021-06-12 where not documented.
This is everything before and including: 4080d574bdd7d86d3061d19bf735a14efd7cd103
