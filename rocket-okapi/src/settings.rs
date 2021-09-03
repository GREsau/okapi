use schemars::gen::SchemaSettings;
use serde::{Deserialize, Serialize};

/// Settings which are used to customise the behaviour of the `OpenApiGenerator`.
#[derive(Debug, Clone)]
pub struct OpenApiSettings {
    /// Settings to customise how JSON Schemas are generated.
    pub schema_settings: SchemaSettings,
    /// The path to the json file that contains the API specification. Then default is
    /// `openapi.json`.
    pub json_path: String,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        OpenApiSettings {
            schema_settings: SchemaSettings::openapi3(),
            json_path: "/openapi.json".to_owned(),
        }
    }
}

impl OpenApiSettings {
    /// Create a new instance of `OpenApiSettings`. Equivalent to calling `Default::default`.
    #[must_use]
    pub fn new() -> Self {
        OpenApiSettings {
            ..OpenApiSettings::default()
        }
    }
}

/// Contains a named url.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlObject {
    /// The name of the url.
    pub name: String,
    /// The url itself.
    pub url: String,
}

impl UrlObject {
    /// Create a new `UrlObject` from the provided name and url.
    #[must_use]
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
        }
    }
}
