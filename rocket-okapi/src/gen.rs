use crate::settings::OpenApiSettings;
use crate::OperationInfo;
use okapi::openapi3::*;
use rocket::http::Method;
use schemars::gen::SchemaGenerator;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use schemars::{Map, MapEntry};
use std::collections::HashMap;
use std::iter::FromIterator;

/// A struct that visits all `rocket::Route`s, and aggregates information about them.
#[derive(Debug, Clone)]
pub struct OpenApiGenerator {
    settings: OpenApiSettings,
    schema_generator: SchemaGenerator,
    operations: Map<String, HashMap<Method, Operation>>,
}

impl OpenApiGenerator {
    /// Create a new `OpenApiGenerator` from the settings provided.
    pub fn new(settings: OpenApiSettings) -> Self {
        OpenApiGenerator {
            schema_generator: settings.schema_settings.clone().into_generator(),
            settings,
            operations: Default::default(),
        }
    }

    /// Add a new `HTTP Method` to the collection of endpoints in the `OpenApiGenerator`.
    pub fn add_operation(&mut self, mut op: OperationInfo) {
        if let Some(op_id) = op.operation.operation_id {
            // TODO do this outside add_operation
            op.operation.operation_id = Some(op_id.trim_start_matches(':').replace("::", "_"));
        }
        match self.operations.entry(op.path.clone()) {
            MapEntry::Occupied(mut e) => {
                let map = e.get_mut();
                if map.insert(op.method, op.operation).is_some() {
                    // This will trow a warning if 2 routes have the same path and method
                    // This is allowed by Rocket when a ranking is given for example: `#[get("/user", rank = 2)]`
                    // See: https://rocket.rs/v0.4/guide/requests/#forwarding
                    println!("Warning: Operation replaced for {}:{}", op.method, op.path);
                }
            }
            MapEntry::Vacant(e) => {
                let mut map = HashMap::new();
                map.insert(op.method, op.operation);
                e.insert(map);
            }
        };
    }

    /// Returns a JSON Schema object for the type `T`.
    pub fn json_schema<T: ?Sized + JsonSchema>(&mut self) -> SchemaObject {
        self.schema_generator.subschema_for::<T>().into()
    }

    /// Obtain the internal `SchemaGenerator` object.
    pub fn schema_generator(&self) -> &SchemaGenerator {
        &self.schema_generator
    }

    /// Return the component definition/schema of an object without any references.
    pub fn json_schema_no_ref<T: ?Sized + JsonSchema>(&mut self) -> SchemaObject {
        <T>::json_schema(&mut self.schema_generator).into()
    }

    /// Generate an `OpenApi` specification for all added operations.
    pub fn into_openapi(self) -> OpenApi {
        OpenApi {
            openapi: "3.0.0".to_owned(),
            paths: {
                let mut paths = Map::new();
                for (path, map) in self.operations {
                    for (method, op) in map {
                        let path_item = paths.entry(path.clone()).or_default();
                        set_operation(path_item, method, op);
                    }
                }
                paths
            },
            components: Some(Components {
                schemas: Map::from_iter(
                    self.schema_generator
                        .into_definitions()
                        .into_iter()
                        .map(|(k, v)| (k, v.into())),
                ),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

fn set_operation(path_item: &mut PathItem, method: Method, op: Operation) {
    use Method::*;
    let option = match method {
        Get => &mut path_item.get,
        Put => &mut path_item.put,
        Post => &mut path_item.post,
        Delete => &mut path_item.delete,
        Options => &mut path_item.options,
        Head => &mut path_item.head,
        Patch => &mut path_item.patch,
        Trace => &mut path_item.trace,
        // Connect not available in OpenAPI3. Maybe should set in extensions?
        Connect => return,
    };
    assert!(option.is_none());
    option.replace(op);
}
