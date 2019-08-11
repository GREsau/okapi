use crate::OperationInfo;
use okapi::openapi3::*;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::{schema::Schema, JsonSchema};

#[derive(Debug, PartialEq, Clone)]
pub struct OpenApiSettings {
    pub schema_settings: SchemaSettings,
}

impl Default for OpenApiSettings {
    fn default() -> Self {
        OpenApiSettings {
            schema_settings: SchemaSettings::openapi3(),
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

#[derive(Debug, Clone)]
pub struct OpenApiGenerator {
    schema_generator: SchemaGenerator,
}

impl OpenApiGenerator {
    pub fn new(settings: OpenApiSettings) -> Self {
        OpenApiGenerator {
            schema_generator: settings.schema_settings.into_generator(),
        }
    }

    pub fn add_operation(&mut self, _op: OperationInfo) {
        // unimplemented!()
    }

    pub fn json_schema<T: ?Sized + JsonSchema>(&mut self) -> schemars::Result<RefOr<SchemaObject>> {
        let schema = self.schema_generator.subschema_for::<T>()?;
        self.get_ref_or_object(schema)
    }

    pub fn get_ref_or_object(&self, schema: Schema) -> schemars::Result<RefOr<SchemaObject>> {
        Ok(match schema {
            Schema::Ref(r) => RefOr::Ref(r),
            schema => self.schema_generator.get_schema_object(&schema)?.into(),
        })
    }

    pub fn schema_generator(&self) -> &SchemaGenerator {
        &self.schema_generator
    }

    pub fn into_openapi(self) -> OpenApi {
        // unimplemented!()
        Default::default()
    }
}
