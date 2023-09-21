use std::error::Error;

use actix_web::HttpResponse;
use diesel::NotFound;
use serde::Serialize;

#[derive(Serialize)]
struct __Error {
    pub error: String,
}

pub fn diesel_error(e: diesel::result::Error) -> HttpResponse {
    match e {
        NotFound => http_not_found(&e),
        _ => http_internal_server_error(&e)
    }
}

#[allow(dead_code)]
fn http_not_found(e: &dyn Error) -> HttpResponse {
    HttpResponse::NotFound()
        .content_type(crate::constants::APPLICATION_JSON)
        .json(__Error { error: e.to_string() })
}

#[allow(dead_code)]
fn http_bad_request(e: &dyn Error) -> HttpResponse {
    HttpResponse::BadRequest()
        .content_type(crate::constants::APPLICATION_JSON)
        .json(__Error { error: e.to_string() })
}

#[allow(dead_code)]
fn http_internal_server_error(e: &dyn Error) -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type(crate::constants::APPLICATION_JSON)
        .json(__Error { error: e.to_string() })
}