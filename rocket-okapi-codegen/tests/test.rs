#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket_okapi_codegen;
#[macro_use]
extern crate rocket;

#[okapi(skip)]
#[get("/a")]
pub fn hi() {}
