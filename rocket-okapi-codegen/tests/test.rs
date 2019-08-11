#[macro_use]
extern crate rocket_okapi_codegen;

#[get("/a", format = "asd")]
pub fn hi() {}
