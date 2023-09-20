use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use crate::constants::APPLICATION_JSON;
use crate::web::error::http_internal_server_error;
use crate::repositories::location;
use super::types::WDPool;


fn http_ok_json<T: Serialize>(json: T) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(json)
}

#[get("/locations")]
pub async fn location_list(pool: WDPool) -> HttpResponse {
    match location::find_all(&pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}

#[get("/items/{id}")]
pub async fn location_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match location::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}