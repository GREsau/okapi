use schemars::gen::SchemaSettings;

#[derive(Debug, PartialEq, Clone)]
pub struct OpenApiSettings {
    pub schema_settings: SchemaSettings,
    pub path: String,
    pub json_path: String,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        OpenApiSettings {
            schema_settings: SchemaSettings::openapi3(),
            path: "/swagger/".to_owned(),
            json_path: "swagger.json".to_owned(),
        }
    }
}

impl OpenApiSettings {
    pub fn new() -> Self {
        OpenApiSettings {
            ..Default::default()
        }
    }

    pub fn full_json_path(&self) -> String {
        format!("{}/{}", self.path, self.json_path)
    }
}
