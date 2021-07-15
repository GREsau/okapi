use super::{OpenApiFromRequest, RequestHeaderInput};
use crate::gen::OpenApiGenerator;

impl<'a, T: Send + Sync> OpenApiFromRequest<'a> for &'a rocket::State<T> {
    fn request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
    ) -> crate::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
