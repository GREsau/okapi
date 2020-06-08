use schemars::gen::SchemaSettings;

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
    pub fn new() -> Self {
        OpenApiSettings {
            ..Default::default()
        }
    }
}
