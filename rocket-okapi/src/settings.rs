use okapi::{Map, openapi3::{RefOr, SecurityScheme, SecuritySchemeData}};
use schemars::gen::SchemaSettings;

/// Settings which are used to customise the behaviour of the `OpenApiGenerator`.
#[derive(Debug, Clone)]
pub struct OpenApiSettings {
    /// Settings to customise how JSON Schemas are generated.
    pub schema_settings: SchemaSettings,
    /// The path to the json file that contains the API specification. Then default is
    /// `openapi.json`.
    pub json_path: String,
    /// SecuritySchemes to be added to the api
    pub security_schemes: Map<String, RefOr<SecurityScheme>>,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        OpenApiSettings {
            schema_settings: SchemaSettings::openapi3(),
            json_path: "/openapi.json".to_owned(),
            security_schemes: Map::new(),
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

    /// Adds a `SecurityScheme` to an instance of `OpenApiSettings`
    pub fn add_security_scheme(&mut self, name: String, security:SecuritySchemeData, description: Option<String>){
        self.security_schemes.insert(name, RefOr::Object{0:SecurityScheme{description:description, extensions:Map::new(), data: security}});
    }

}
