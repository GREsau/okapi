#![feature(prelude_import)]
#![no_std]
#![feature(proc_macro_hygiene , decl_macro)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

#[macro_use]
extern crate rocket;

use okapi::openapi3;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;

// pub struct RouteInfo {
//     pub route:
// }

//#[macro_use]
//extern crate rocket_okapi;

//#[okapi]
fn index() -> Json<&'static str> { Json("Hello, world!") }
#[doc = r" Rocket code generated wrapping route function."]
fn rocket_route_fn_index<'_b>(__req: &'_b ::rocket::Request,
                              __data: ::rocket::Data)
 -> ::rocket::handler::Outcome<'_b> {
    let ___responder = index();
    ::rocket::handler::Outcome::from(__req, ___responder)
}
#[doc = r" Rocket code generated wrapping URI macro."]
macro rocket_uri_macro_index {
    ($ ($ token : tt) *) =>
    {
        {
            extern crate std ; extern crate rocket ; rocket ::
            rocket_internal_uri ! ("/" , () , $ ($ token) *)
        }
    }
}
#[doc = r" Rocket code generated static route info."]
#[allow(non_upper_case_globals)]
static static_rocket_route_info_for_index: ::rocket::StaticRouteInfo =
    ::rocket::StaticRouteInfo{name: "index",
                              method: ::rocket::http::Method::Get,
                              path: "/",
                              handler: rocket_route_fn_index,
                              format: None,
                              rank: None,};

//#[okapi]
//#[okapi(200 => &str)]
//#[okapi(404 => ())]
//#[okapi(401 => (), "Authentication failed.")]
fn loud() -> Json<&'static str> { Json("I AM SHOUTING!!!!!") }
#[doc = r" Rocket code generated wrapping route function."]
fn rocket_route_fn_loud<'_b>(__req: &'_b ::rocket::Request,
                             __data: ::rocket::Data)
 -> ::rocket::handler::Outcome<'_b> {
    let ___responder = loud();
    ::rocket::handler::Outcome::from(__req, ___responder)
}
#[doc = r" Rocket code generated wrapping URI macro."]
macro rocket_uri_macro_loud {
    ($ ($ token : tt) *) =>
    {
        {
            extern crate std ; extern crate rocket ; rocket ::
            rocket_internal_uri ! ("/loud" , () , $ ($ token) *)
        }
    }
}
#[doc = r" Rocket code generated static route info."]
#[allow(non_upper_case_globals)]
static static_rocket_route_info_for_loud: ::rocket::StaticRouteInfo =
    ::rocket::StaticRouteInfo{name: "loud",
                              method: ::rocket::http::Method::Get,
                              path: "/loud",
                              handler: rocket_route_fn_loud,
                              format: None,
                              rank: None,};

//#[okapi]
fn to_number(value: String)
 -> Result<Json<f64>, NotFound<Json<&'static str>>> {
    match value.parse() {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(NotFound(Json("That's not a number!"))),
    }
}
#[doc = r" Rocket code generated wrapping route function."]
fn rocket_route_fn_to_number<'_b>(__req: &'_b ::rocket::Request,
                                  __data: ::rocket::Data)
 -> ::rocket::handler::Outcome<'_b> {
    #[allow(non_snake_case , unreachable_patterns , unreachable_code)]
    let __rocket_param_value: String =
        match __req.raw_segment_str(1usize) {
            Some(__s) =>
            match <String as ::rocket::request::FromParam>::from_param(__s) {
                Ok(__v) => __v,
                Err(__error) =>
                return {
                           ::rocket::logger::warn_(&
                                                       //#[okapi]

                                                       //#[okapi(skip)]

                                                       // let okapi = OkapiGenerator::new().mount("/", okapi_routes![index, loud, to_number, to_number_post, hidden]).generate("Test API", "0.1");
                                                       // .mount_okapi("/swagger", okapi)
                                                       // or .mount_okapi("/swagger", okapi_routes![index, loud, to_number, to_number_post, hidden])
                                                       ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Failed to parse \'",
                                                                                                            "\': "],
                                                                                                          &match (&"__rocket_param_value",
                                                                                                                  &__error)
                                                                                                               {
                                                                                                               (arg0,
                                                                                                                arg1)
                                                                                                               =>
                                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                            ::std::fmt::Display::fmt),
                                                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                                           })));
                           ::rocket::Outcome::Forward(__data)
                       },
            },
            None =>
            return {
                       ::rocket::logger::error("Internal invariant error: expected dynamic parameter not found.");
                       ::rocket::logger::error("Please report this error to the Rocket issue tracker.");
                       ::rocket::Outcome::Forward(__data)
                   },
        };
    let ___responder = to_number(__rocket_param_value);
    ::rocket::handler::Outcome::from(__req, ___responder)
}
#[doc = r" Rocket code generated wrapping URI macro."]
macro rocket_uri_macro_to_number {
    ($ ($ token : tt) *) =>
    {
        {
            extern crate std ; extern crate rocket ; rocket ::
            rocket_internal_uri !
            ("/tonumber/<value>" , (value : String) , $ ($ token) *)
        }
    }
}
#[doc = r" Rocket code generated static route info."]
#[allow(non_upper_case_globals)]
static static_rocket_route_info_for_to_number: ::rocket::StaticRouteInfo =
    ::rocket::StaticRouteInfo{name: "to_number",
                              method: ::rocket::http::Method::Get,
                              path: "/tonumber/<value>",
                              handler: rocket_route_fn_to_number,
                              format: None,
                              rank: None,};
fn to_number_post(value: Json<String>)
 -> Result<Json<f64>, NotFound<Json<&'static str>>> {
    match value.parse() {
        Ok(f) => Ok(Json(f)),
        Err(_) => Err(NotFound(Json("That's not a number!"))),
    }
}
#[doc = r" Rocket code generated wrapping route function."]
fn rocket_route_fn_to_number_post<'_b>(__req: &'_b ::rocket::Request,
                                       __data: ::rocket::Data)
 -> ::rocket::handler::Outcome<'_b> {
    let __transform =
        <Json<String> as ::rocket::data::FromData>::transform(__req, __data);
    #[allow(unreachable_patterns , unreachable_code)]
    let __outcome =
        match __transform {
            ::rocket::data::Transform::Owned(::rocket::Outcome::Success(__v))
            => {
                ::rocket::data::Transform::Owned(::rocket::Outcome::Success(__v))
            }
            ::rocket::data::Transform::Borrowed(::rocket::Outcome::Success(ref __v))
            => {
                ::rocket::data::Transform::Borrowed(::rocket::Outcome::Success(::std::borrow::Borrow::borrow(__v)))
            }
            ::rocket::data::Transform::Borrowed(__o) =>
            ::rocket::data::Transform::Borrowed(__o.map(|_|
                                                            {
                                                                {
                                                                    {
                                                                        {
                                                                            ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                                                                      &match (&"Borrowed(Success(..)) case handled in previous block",)
                                                                                                                                           {
                                                                                                                                           (arg0,)
                                                                                                                                           =>
                                                                                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                        ::std::fmt::Display::fmt)],
                                                                                                                                       }),
                                                                                                       &("src\\main.rs",
                                                                                                         43u32,
                                                                                                         19u32))
                                                                        }
                                                                    }
                                                                }
                                                            })),
            ::rocket::data::Transform::Owned(__o) =>
            ::rocket::data::Transform::Owned(__o),
        };
    #[allow(non_snake_case , unreachable_patterns , unreachable_code)]
    let __rocket_param_value: Json<String> =
        match <Json<String> as
                  ::rocket::data::FromData>::from_data(__req, __outcome) {
            ::rocket::Outcome::Success(__d) => __d,
            ::rocket::Outcome::Forward(__d) =>
            return ::rocket::Outcome::Forward(__d),
            ::rocket::Outcome::Failure((__c, _)) =>
            return ::rocket::Outcome::Failure(__c),
        };
    let ___responder = to_number_post(__rocket_param_value);
    ::rocket::handler::Outcome::from(__req, ___responder)
}
#[doc = r" Rocket code generated wrapping URI macro."]
macro rocket_uri_macro_to_number_post {
    ($ ($ token : tt) *) =>
    {
        {
            extern crate std ; extern crate rocket ; rocket ::
            rocket_internal_uri ! ("/tonumber" , () , $ ($ token) *)
        }
    }
}
#[doc = r" Rocket code generated static route info."]
#[allow(non_upper_case_globals)]
static static_rocket_route_info_for_to_number_post: ::rocket::StaticRouteInfo
       =
    ::rocket::StaticRouteInfo{name: "to_number_post",
                              method: ::rocket::http::Method::Post,
                              path: "/tonumber",
                              handler: rocket_route_fn_to_number_post,
                              format: None,
                              rank: None,};
fn hidden() -> Json<&'static str> { Json("Hidden from swagger!") }
#[doc = r" Rocket code generated wrapping route function."]
fn rocket_route_fn_hidden<'_b>(__req: &'_b ::rocket::Request,
                               __data: ::rocket::Data)
 -> ::rocket::handler::Outcome<'_b> {
    let ___responder = hidden();
    ::rocket::handler::Outcome::from(__req, ___responder)
}
#[doc = r" Rocket code generated wrapping URI macro."]
macro rocket_uri_macro_hidden {
    ($ ($ token : tt) *) =>
    {
        {
            extern crate std ; extern crate rocket ; rocket ::
            rocket_internal_uri ! ("/hidden" , () , $ ($ token) *)
        }
    }
}
#[doc = r" Rocket code generated static route info."]
#[allow(non_upper_case_globals)]
static static_rocket_route_info_for_hidden: ::rocket::StaticRouteInfo =
    ::rocket::StaticRouteInfo{name: "hidden",
                              method: ::rocket::http::Method::Get,
                              path: "/hidden",
                              handler: rocket_route_fn_hidden,
                              format: None,
                              rank: None,};
fn main() {
    rocket::ignite().mount("/",
                           {
                               let __vector: Vec<::rocket::Route> =
                                   <[_]>::into_vec(box
                                                       [::rocket::Route::from(&static_rocket_route_info_for_index),
                                                        ::rocket::Route::from(&static_rocket_route_info_for_loud),
                                                        ::rocket::Route::from(&static_rocket_route_info_for_to_number),
                                                        ::rocket::Route::from(&static_rocket_route_info_for_to_number_post),
                                                        ::rocket::Route::from(&static_rocket_route_info_for_hidden)]);
                               __vector
                           }).launch();
}
