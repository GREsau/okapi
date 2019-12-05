use crate::settings::OpenApiSettings;
use crate::OperationInfo;
use okapi::openapi3::*;
use okapi::Map;
use rocket::http::Method;
use schemars::gen::SchemaGenerator;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use std::collections::{hash_map::Entry as HashEntry, HashMap};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct OpenApiGenerator {
    settings: OpenApiSettings,
    schema_generator: SchemaGenerator,
    operations: HashMap<(String, Method), Operation>,
}

impl OpenApiGenerator {
    pub fn new(settings: OpenApiSettings) -> Self {
        OpenApiGenerator {
            schema_generator: settings.schema_settings.clone().into_generator(),
            settings,
            operations: Default::default(),
        }
    }

    pub fn add_operation(&mut self, mut op: OperationInfo) {
        if let Some(op_id) = op.operation.operation_id {
            // TODO do this outside add_operation
            op.operation.operation_id = Some(op_id.trim_start_matches(':').replace("::", "_"));
        }
        match self.operations.entry((op.path, op.method)) {
            HashEntry::Occupied(e) => {
                let (path, method) = e.key();
                panic!(
                    "An OpenAPI operation has already been added for {} {}",
                    method, path
                );
            }
            HashEntry::Vacant(e) => e.insert(op.operation),
        };
    }

    pub fn json_schema<T: ?Sized + JsonSchema>(&mut self) -> SchemaObject {
        self.schema_generator.subschema_for::<T>().into()
    }

    pub fn schema_generator(&self) -> &SchemaGenerator {
        &self.schema_generator
    }

    pub fn into_openapi(self) -> OpenApi {
        OpenApi {
            openapi: "3.0.0".to_owned(),
            paths: {
                let mut paths = Map::new();
                for ((path, method), op) in self.operations {
                    let path_item = paths.entry(path).or_default();
                    set_operation(path_item, method, op);
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
