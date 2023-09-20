use std::error::Error;
use actix_web::HttpResponse;
use serde::{Serialize};

#[derive(Serialize)]
struct __Error {
    pub error: String
}

#[allow(dead_code)]
pub fn http_bad_request(e: &dyn Error) -> HttpResponse {
    HttpResponse::BadRequest()
        .content_type(crate::constants::APPLICATION_JSON)
        .json(__Error { error: e.to_string() })
}

pub fn http_internal_server_error(e: &dyn Error) -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type(crate::constants::APPLICATION_JSON)
        .json(__Error { error: e.to_string() })
}