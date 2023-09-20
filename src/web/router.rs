use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use crate::constants::APPLICATION_JSON;
use crate::web::error::http_internal_server_error;
use crate::repositories::{containers, groups, locations};
use super::types::WDPool;

fn http_ok_json<T: Serialize>(json: T) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(json)
}

#[get("/location")]
pub async fn location_list(pool: WDPool) -> HttpResponse {
    match locations::find_all(&pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}

#[get("/location/{id}")]
pub async fn location_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match locations::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}

#[get("/group")]
pub async fn group_list(pool: WDPool) -> HttpResponse {
    match groups::find_all(&pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}

#[get("/group/{id}")]
pub async fn group_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match groups::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}

#[get("/container")]
pub async fn container_list(pool: WDPool) -> HttpResponse {
    match containers::find_all(&pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}

#[get("/container/{id}")]
pub async fn container_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match containers::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => http_internal_server_error(&e),
    }
}
