# Change log
All notable changes to this project will be documented in this file.
This project follows the [Semantic Versioning standard](https://semver.org/).

## Unreleased (2021-xx-xx)

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## Version 0.8.0-rc.1 (2021-10-02)

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
- Add support for UUIDs, and added example. (#38, #46, #54)
- Added `log v0.4` as a dependency.
- Added `either v1` as a dependency. (Rocket dependency)
- Added feature flag for [`msgpack`](https://docs.rs/rocket/0.5.0-rc.1/rocket/serde/msgpack/struct.MsgPack.html)
(Re-exposing Rocket feature flag)
- Added support for new [`Responder`](https://docs.rs/rocket/0.5.0-rc.1/rocket/response/trait.Responder.html)
types (implemented `OpenApiResponderInner`):
   - `std::fs::File`
   - `rocket::tokio::fs::File`
   - `std::borrow::Cow<'o, T>`
   - `either::Either<L, R>`
   - `std::io::Error`
   - `(rocket::http::ContentType, R)`
   - `(rocket::http::Status, R)`
   - `rocket::http::Status` (#20)
   - `rocket::response::status::NoContent`
   - `rocket::response::Redirect`
   - `rocket::response::content::Custom<T>`
   - `rocket::response::status::Conflict<T>`
   - `rocket::response::status::Custom<T>`
   - `rocket::response::Flash<R>`
   - `rocket::data::Capped<R>`
   - `rocket::response::Debug<E>`
   - `rocket::response::stream::ByteStream<S>`
   - `rocket::response::stream::ReaderStream<S>`
   - `rocket::response::stream::TextStream<S>`
   - `rocket::serde::msgpack::MsgPack<T>` (only when feature `msgpack` is enabled)
- Fully implement `FromSegments` for `<param..>` in path. (#41)
- Implement `OpenApiFromSegments` for all that implement `FromSegments` and `JsonSchema`. (#41)
- Implement `OpenApiFromParam` for all that implement `FromParam` and `JsonSchema`.
- Implement `OpenApiFromFormField` for all that implement `FromFormField` and `JsonSchema`.
- Added support for new [`FromData`](https://docs.rs/rocket/0.5.0-rc.1/rocket/data/trait.FromData.html)
types (implemented `OpenApiFromData`):
   - `String`
   - `&'r str`
   - `Cow<'r, str>`
   - `Vec<u8>`
   - `&'r [u8]`
   - `rocket::fs::TempFile<'r>`
   - `rocket::data::Capped<rocket::fs::TempFile<'r>>`
   - `rocket::data::Capped<Cow<'r, str>>`
   - `rocket::data::Capped<&'r str>`
   - `rocket::data::Capped<&'r rocket::http::RawStr>`
   - `rocket::data::Capped<&'r [u8]>`
   - `rocket::data::Capped<String>`
   - `rocket::data::Capped<Vec<u8>>`
   - `&'r rocket::http::RawStr`
   - `rocket::form::Form<T>`
   - `rocket::serde::msgpack::MsgPack<T>` (only when feature `msgpack` is enabled)
- Added feature flag for [`secrets`](https://rocket.rs/v0.5-rc/guide/requests/#secret-key)
(Re-exposing Rocket feature flag)
- Added support for [Request Guards](https://rocket.rs/v0.4/guide/requests/#request-guards)
and [Security Scheme](https://swagger.io/docs/specification/authentication/)
(aka Authentication and Authorization) (#47, #9, #8, #56)
- Added support for new [`FromRequest`](https://docs.rs/rocket/0.5.0-rc.1/rocket/request/trait.FromRequest.html)
  types (implemented `OpenApiFromRequest`):
  - `std::net::IpAddr`
  - `std::net::SocketAddr`
  - `Result<T, T::Error>`
  - `Option<T>`
  - `&'r rocket::config::Config`
  - `&'r rocket::config::SecretKey`(only when feature `secrets` is enabled)
  - `&'r rocket::data::Limits`
  - `&'r rocket::http::Accept`
  - `&'r rocket::http::ContentType`
  - `&'r rocket::http::CookieJar<'r>`
  - `&'r rocket::http::uri::Origin<'r>`
  - `&'r rocket::route::Route`
  - `rocket::http::Method`
  - `rocket::Shutdown`
  - `&'r rocket::State<T>`
- Added `OpenApiFromRequest` derive macro.
- Added feature flag for `preserve_order` to keep the order of struct fields in `Schema`
all parts of the `OpenAPI` documentation.

### Changed
- Swagger UI is now only available under the feature `swagger`.
- Updated Swagger UI to v3.52.0
- `UrlObject` has been moved from `swagger_ui::UrlObject` to `settings::UrlObject`.
- Replaced manual implementations of `OpenApiFromParam` with generic version.
So `OpenApiFromParam` is implemented for more types.
- Replaced manual implementations of `OpenApiFromFormField` with generic version.
So `OpenApiFromFormField` is implemented for more types.
- Fixed setting of parameter location from "form" to "query".

### Removed
- Removed unused and unneeded files from Swagger UI to decrease file size.
- `routes_with_openapi` has be removed and replaced with `openapi_routes`.

## Pre version 0.8.0-rc.1 (2021-06-12)
All changes before 2021-06-12 where not documented.
This is everything before and including: 4080d574bdd7d86d3061d19bf735a14efd7cd103
