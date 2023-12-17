# Okapi
Okapi: [![Download](https://img.shields.io/crates/v/okapi)](https://crates.io/crates/okapi/)
[![API Docs](https://img.shields.io/badge/docs-okapi-blue)](https://docs.rs/okapi/latest/okapi/)

Rocket-Okapi: [![Download](https://img.shields.io/crates/v/rocket_okapi)](https://crates.io/crates/rocket_okapi)
[![API Docs](https://img.shields.io/badge/docs-rocket_okapi-blue)](https://docs.rs/rocket_okapi/latest/rocket_okapi/)

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Automated OpenAPI (AKA Swagger) document generation for Rust/Rocket projects.

Never have outdated documentation again.
Okapi will generate documentation for you while setting up the server.
It uses a combination of [Rust Doc comments](https://doc.rust-lang.org/reference/comments.html#doc-comments)
and programming logic to document your API.

The generated [OpenAPI][OpenAPI_3.0.0] files can then be used by various programs to
visualize the documentation. Rocket-okapi currently includes [RapiDoc][RapiDoc] and
[Swagger UI][Swagger_UI], but others can be used too.

Supported OpenAPI Spec: [3.0.0][OpenAPI_3.0.0]<br/>
Supported Rocket version (for `rocket_okapi`): [0.5.0](https://crates.io/crates/rocket/0.5.0)

Example of generated documentation using Okapi:
- DF Storyteller: [RapiDoc](https://docs.dfstoryteller.com/rapidoc/),
[Swagger UI](https://docs.dfstoryteller.com/swagger-ui/)
- ...[^1]

[^1]: More examples will be added, please open an issue if you have a good example.

## Examples
- [Json web API](examples/json-web-api): Simple example showing the basics of Okapi.
- [UUID](examples/uuid): Simple example showing basics, but using UUID's instead of
normal `u32`/`u64` id's.
- [Custom Schema](examples/custom_schema): Shows how to add more/custom info to OpenAPI file
and merge multiple modules into one OpenAPI file.
- [Secure Request Guard](examples/secure_request_guard): Shows how to implement authentication
methods into the OpenAPI file.
It shows: No authentication, API keys, HTTP Auth, OAuth2, OpenID and Cookies.
- [Special types](examples/special-types): Showing use of some more obscure types and there usage.
(Still work in progress)
- [And more](https://github.com/GREsau/okapi/tree/master/examples)

## FAQ

### Q: Can I generate code from my OpenAPI file?
A: No, this crate only allows you to automatically generate the OpenAPI file from your code.
There are other crates that (attempt to) do this.
So:
  - (Rust code (Rocket) --> OpenAPI) == Okapi
  - (OpenAPI --> Rust code) != Okapi

### Q: How do I document my endpoints?
A: Okapi automatically uses the [Rust Doc Comments](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
from most places, this includes:
    - Endpoint functions.
    - Endpoint function arguments, using [Schemars][Schemars]. Adding documentation for `String`
    and other default types is not possible unless used in an other `struct`. See
    [this issue for more info](https://github.com/GREsau/okapi/issues/102#issuecomment-1152918141).
    - Endpoint function return type, using [Schemars][Schemars]. Same rules apply as arguments.
    In case of `Result<T, E>`, the error codes can be documented,
    [see this example](https://github.com/GREsau/okapi/blob/master/examples/custom_schema/src/error.rs).

Some more info can be provided using the `#[openapi(...)]` derive macro, for more info see:
[OpenApiAttribute](https://github.com/GREsau/okapi/blob/master/rocket-okapi-codegen/src/openapi_attr/mod.rs#L22).

[Schemars][Schemars] also can be used to provide more info for objects that implement
`#[derive(JsonSchema)]` using the `#[schemars(...)]` and `#[serde(...)]` syntax.
[For more info see `Attrs`](https://github.com/GREsau/schemars/blob/master/schemars_derive/src/attr/mod.rs#L22)

Documentation can be enhanced in most other places too, but might require custom implementations.
[See our examples for more info](https://github.com/GREsau/okapi/tree/master/examples).

If the above is not sufficient, you can always create your custom
[`OpenAPI`](https://docs.rs/okapi/latest/okapi/openapi3/struct.OpenApi.html) objects.
This will can then be merged into the final OpenAPI file.
[For more info see this example](https://github.com/GREsau/okapi/blob/master/examples/custom_schema/src/main.rs#L61).
Use this method only if really necessary! (As it might overwrite other generated objects.)

### Q: My (diesel) database does not implement `OpenApiFromRequest`.
A: This is because the parameter does not show up in the path, query or body.
So this is considered a [Request Guard](https://rocket.rs/v0.5/guide/requests/#request-guards).
There is a [derive macro](https://github.com/GREsau/okapi/blob/master/examples/secure_request_guard/src/no_auth.rs)
for this, but this does not work in combination with the `#[database("...")]` marco.
You can solve this my implementing it manually, like this:
<details>
    <summary>Implement `OpenApiFromRequest` for Diesel DB</summary>

```rust
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_sync_db_pools::{diesel, database};

#[database("sqlite_logs")]
pub struct MyDB(diesel::SqliteConnection);

impl<'r> OpenApiFromRequest<'r> for MyDB {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
```
</details>

### Q: ... does not implement `JsonSchema`?
A: The [`JsonSchema`](https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html) implementation
is handled by [`Schemars`][Schemars], make sure you enabled the right
[feature flags](https://github.com/GREsau/schemars#optional-dependencies) for it.
If it is still not implemented open an issue in the `Schemars` repo.

### Q: Can I add custom data to my OpenAPI spec?
A: Yes, see the [Custom Schema](examples/custom_schema) example. Okapi also has build in functions
if you want to merge the [`OpenAPI`](https://docs.rs/okapi/latest/okapi/openapi3/struct.OpenApi.html)
objects manually.

### Q: Can I use this with other web frameworks then Rocket?
A: Yes, but not there are no other implementations right now. But you can use the `Okapi` crate
independently and use Serde to create the json or yaml file.

## Feature Flags
Okapi:
- `impl_json_schema`: Implements [`JsonSchema`](https://docs.rs/schemars/latest/schemars/trait.JsonSchema.html)
for [`Schemars`][Schemars] and `Okapi` types themselves.
- `preserve_order`: Keep the order of struct fields in `Schema` and all parts of the
`OpenAPI` documentation.

Rocket-Okapi:
- `preserve_order`: Keep the order of struct fields in `Schema` and all parts of the
`OpenAPI` documentation.
- `swagger`: Enable [Swagger UI][Swagger_UI] for rendering documentation.
- `rapidoc`: Enable [RapiDoc][RapiDoc] for rendering documentation.
- `uuid`: Enable UUID support in Rocket and Schemars.
- `msgpack`: Enable [msgpack support for Rocket](https://docs.rs/rocket/latest/rocket/serde/msgpack/struct.MsgPack.html).
(when same Rocket feature flag is used.)
- `secrets`: Enable [secrets support for Rocket](https://rocket.rs/v0.5/guide/requests/#secret-key).
(when same Rocket feature flag is used.)
- `mtls`: Enable [mutual TSL for Rocket](https://rocket.rs/v0.5/guide/configuration/#mutual-tls).
(when same Rocket feature flag is used.)
- `rocket_dyn_templates`: Enable compatibility with [`rocket_dyn_templates`](https://crates.io/crates/rocket_dyn_templates).
- `rocket_db_pools`: Enable compatibility with [`rocket_db_pools`](https://crates.io/crates/rocket_db_pools).
- `rocket_sync_db_pools`: Enable compatibility with [`rocket_sync_db_pools`](https://crates.io/crates/rocket_sync_db_pools).
- `rocket_ws`: Enable compatibility with [`rocket_ws`](https://crates.io/crates/rocket_ws).

Note that not all feature flags from [`Schemars`][Schemars] are re-exported or enabled.
So if you have objects for which the `JsonSchema` trait is not implemented,
you might need to enable a [feature flag in `Schemars`](https://github.com/GREsau/schemars#optional-dependencies).
For an example see [the "uuid1" example](examples/uuid/Cargo.toml). (Make sure crate versions match)

## How it works
This crate automatically generates an OpenAPI file when the Rocket server starts.
The way this is done is shortly described here.

The [`Schemars`][Schemars] crate provides us with the schemas for all the different
structures and enums. Okapi does not implement any schemas directly, this is all handled by `Schemars`.

The `Okapi` crate just contains all the structures needed to create an OpenAPI file.
This crate does not contain any code for the creation of them, just the structure and code to merge
two [`OpenAPI`](https://docs.rs/okapi/latest/okapi/openapi3/struct.OpenApi.html) structured together.
This crate can be reused to create OpenAPI support in other web framework.

`Rocket-Okapi` crate contains all the code for generating the OpenAPI file and serve it once created.
This code is usually executed using macro's like: [`mount_endpoints_and_merged_docs!{...}`, 
`openapi_get_routes![...]`, `openapi_get_routes_spec![...]` and `openapi_get_spec![...]`
](https://docs.rs/rocket_okapi/latest/rocket_okapi/#macros).

When the Rocket server is started (or wherever macro is placed) the OpenAPI file is generated once.
This file/structure is then stored in memory and will be served when requested.

The `Rocket-Okapi-codegen` crate contains code for
[derive macros](https://doc.rust-lang.org/book/ch19-06-macros.html). 
`#[openapi]`, `rocket_okapi::openapi_spec![...]`, `rocket_okapi::openapi_routes![...]`
and `#[derive(OpenApiFromRequest)]` in our case.
This needs to be in a separate crate because of Rust restrictions.
Note: `derive` or `codegen` crates are usually a bit hard to work with then other crates.
So it is recommended to get some experience with how derive macros work before you
change things in here.

## TODO
- [ ] Tests
- [ ] Documentation
- [ ] Benchmark/optimise memory usage and allocations
  - Note to self: https://crates.io/crates/graphannis-malloc_size_of looks useful
- [x] Implement `OpenApiFrom___`/`OpenApiResponder` for more rocket/rocket-contrib types
- [x] Allow customizing openapi generation settings, e.g.
    - [x] custom json schema generation settings
    - [x] change path the document is hosted at

## License
This project is licensed under the [MIT license](LICENSE).

All contributions to this project will be similarly licensed.

[Schemars]: https://github.com/GREsau/schemars
[OpenAPI_3.0.0]: https://spec.openapis.org/oas/v3.0.0
[RapiDoc]: https://mrin9.github.io/RapiDoc/
[Swagger_UI]: https://swagger.io/tools/swagger-ui/