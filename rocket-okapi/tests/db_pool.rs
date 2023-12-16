//! This test should just compile.

use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_sync_db_pools::{database, diesel};

#[database("sqlite_logs")]
pub struct MyDB(diesel::SqliteConnection);

impl<'r> OpenApiFromRequest<'r> for MyDB {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
