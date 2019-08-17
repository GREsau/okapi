use schemars::gen::SchemaSettings;

#[derive(Debug, PartialEq, Clone)]
pub struct OpenApiSettings {
    pub schema_settings: SchemaSettings,
    pub json_path: String,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        OpenApiSettings {
            schema_settings: SchemaSettings::openapi3(),
            json_path: "/openapi/openapi.json".to_owned(),
        }
    }
}

impl OpenApiSettings {
    pub fn new() -> Self {
        OpenApiSettings {
            ..Default::default()
        }
    }
}
