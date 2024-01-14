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

## Version 0.8.0 (2024-01-14)

### Added
- Added support for new [`Responder`](https://docs.rs/rocket/0.5.0/rocket/response/trait.Responder.html)
  types (implemented `OpenApiResponderInner`):
  - `Box<T>`
- Added support for new [`FromRequest`](https://docs.rs/rocket/0.5.0/rocket/request/trait.FromRequest.html)
  types (implemented `OpenApiFromRequest`):
  - `rocket::request::Outcome<T, T::Error>`
- New feature flag `rocket_sync_db_pools` for compatibility with
  [`rocket_sync_db_pools`](https://crates.io/crates/rocket_sync_db_pools).
- New feature flag `rocket_ws` for compatibility with [`rocket_ws`](https://crates.io/crates/rocket_ws).
- Added new example for WebSockets.
- Added support for new [`Responder`](https://docs.rs/rocket/0.5.0/rocket/response/trait.Responder.html)
  types (implemented `OpenApiResponderInner`):
  - `rocket_ws::Channel<'o>` (when `rocket_ws` feature is enabled)
  - `rocket_ws::stream::MessageStream<'o, S>` (when `rocket_ws` feature is enabled)
- Added support for new [`FromRequest`](https://docs.rs/rocket/0.5.0/rocket/request/trait.FromRequest.html)
  types (implemented `OpenApiFromRequest`):
  - `rocket_dyn_templates::Metadata<'r>` (when `rocket_dyn_templates` feature is enabled)
  - `rocket_sync_db_pools::example::ExampleDb` (when `rocket_sync_db_pools` feature is enabled)
  - `rocket_ws::WebSocket` (when `rocket_ws` feature is enabled)
- Added `get_nested_endpoints_and_docs` to support more module based APIs. (#137, #138)

### Changed
- `rocket-okapi` and `rocket-okapi-codegen` require `rocket v0.5.0`. (#132)
- Updated RapiDoc to `v9.3.4` and updated settings, including changed defaults in RapiDocs.
    - Schema settings moved to `SchemaConfig`.
    - `GeneralConfig` added: `update_route`, `route_prefix` and `persist_auth`.
    - `UiConfig` added: `css_file` and `css_classes`.
    - `NavConfig` added: `show_method_in_nav_bar`, `nav_accent_text_color`, `nav_active_item_marker`
      and `on_nav_tag_click`.
    - `NavConfig` removed: `nav_bg_image`, `nav_bg_image_size` and `nav_bg_image_repeat`.
    - `HideShowConfig` added: `allow_spec_file_download`, `allow_advanced_search` and `show_curl_before_try`.
    - `ApiConfig` changed: `api_key_name`, `api_key_location` and `fetch_credentials`.
    - `SlotsConfig` added: `operations_top`.
    - Note: The default `RenderStyle` changed from `View` to `Read`.
    - Note: `schema_hide_read_only` and `schema_hide_write_only` changed behavior.
- Updated Swagger UI to `v5.11.0`.
- Updated all crates to Rust 2021 edition.
- Renamed `uuid` example to `uuid_usage`, so it does not collide with `uuid` crate.

This version also includes all the changes from `0.8.0-rc.1`, `0.8.0-rc.2` and `0.8.0-rc.3`.
See below for more info on the changes made in these versions.

## Version 0.8.0-rc.3 (2023-05-29)

### Added
- Add `ignore` derive attribute to ignore function arguments from documentation. (#113)
- `operation_id` can now be overwritten using `#[openapi(operation_id = "my_custom_id")]`,
but it has to be unique in the API spec. (#63)
- Added support for `#[openapi(deprecated)]` to mark endpoint as deprecated in spec. (#123)

### Changed
- `rocket-okapi` and `rocket-okapi-codegen` require `rocket v0.5.0-rc.3`. (#122)

### Fixed
- `mount_endpoints_and_merged_docs!` does avoid combined paths with double `/`.
- Raw identifiers (`r#type`) can now be used in parameters. (#117)
- `rocket::form::Form<T>` now set requested data as "multipart/form-data". (#80)

## Version 0.8.0-rc.2 (2022-06-07)

Note that this update is a minor version update, but still contains breaking changes because the
Rocket version had a minor update with breaking changes too. This will thus fix previous error
because of incompatibility.

### Added
- Add support for `rocket::response::stream::EventStream<S>` (#52)
- Update Rocket from `0.5.0-rc.1` to `0.5.0-rc.2`. (#89)
- Updated Swagger UI to `v4.12.0`.
- Updated RapiDoc to `v9.3.2`.
- New Rocket feature flags `mtls` re-exposed.
- Added support for new [`Responder`](https://docs.rs/rocket/0.5.0-rc.2/rocket/response/trait.Responder.html)
  types (implemented `OpenApiResponderInner`):
  - `Arc<str>`
  - `Arc<[u8]>`
  - `Box<[u8]>`
  - `Box<str>`
  - `rocket::response::Redirect` adds `500 Internal Server Error` status code.
  - `rocket_dyn_templates::Template` (requires `rocket_dyn_templates` feature)
    ([See example](../examples/dyn_templates/src/main.rs))
  - Some other changes because of renamed types in Rocket.
- Added support for new [`FromRequest`](https://docs.rs/rocket/0.5.0-rc.2/rocket/request/trait.FromRequest.html)
  types (implemented `OpenApiFromRequest`):
  - `rocket::http::uri::Host`
  - `Certificate` (when `mtls` feature is enabled)
  - `FlashMessage`
  - `rocket_db_pools::Connection<D>` (when `rocket_db_pools` feature is enabled) (#104)
- New feature flag `rocket_dyn_templates` for enable compatibility with
[`rocket_dyn_templates`](https://crates.io/crates/rocket_dyn_templates).
- New feature flag `rocket_db_pools` for enable compatibility with
[`rocket_db_pools`](https://crates.io/crates/rocket_db_pools).
- New example for Rocket's Dynamic Templates.

### Changed
- Changed `Data<'r>` from `String` type is binary data (`Vec<u8>`) in `FromData` implementation. (#65)
- Fixed missing of schema for `EventStream` and `TextStream`. (#86)
- Generated functions are no longer included in Rust Documentation. (#69)

### Fixed
- Response schema added for `Vec<u8>`, `&[u8]`, `std::fs::File` and other octet-streams. (#72)
- Fix support for Streams: (#68)
   - `rocket::response::stream::ByteStream<S>`
   - `rocket::response::stream::ReaderStream<S>`
   - `rocket::response::stream::TextStream<S>`

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
(aka Authentication and Authorization) (#47, #9, #8, #56)
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
- `Okapi` create is now re-exported and can be found as `rocket_okapi::okapi`.

### Changed
- Swagger UI is now only available under the feature `swagger`.
- Updated Swagger UI to v3.52.0
- The `preserve_order` feature flag is now enabled by default.
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
