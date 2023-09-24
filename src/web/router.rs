use actix_web::{get, HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_JSON;
use crate::repositories::{categories, containers, groups, items, kinds, locations};
use crate::web::error::diesel_error;

use super::types::WDPool;

#[derive(Debug, Deserialize)]
pub struct Pageable {
    pub start: Option<u32>,
    pub size: Option<u32>,
}

fn http_ok_json<T: Serialize>(json: T) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(json)
}

#[get("/location")]
pub async fn location_list(page: web::Query<Pageable>, pool: WDPool) -> HttpResponse {
    match locations::find_all(page.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/location/{id}")]
pub async fn location_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match locations::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/group")]
pub async fn group_list(page: web::Query<Pageable>, pool: WDPool) -> HttpResponse {
    match groups::find_all(page.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/group/{id}")]
pub async fn group_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match groups::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/container")]
pub async fn container_list(page: web::Query<Pageable>, pool: WDPool) -> HttpResponse {
    match containers::find_all(page.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/container/{id}")]
pub async fn container_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match containers::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/kind")]
pub async fn kind_list(page: web::Query<Pageable>, pool: WDPool) -> HttpResponse {
    match kinds::find_all(page.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/kind/{id}")]
pub async fn kind_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match kinds::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/category")]
pub async fn category_list(page: web::Query<Pageable>, pool: WDPool) -> HttpResponse {
    match categories::find_all(page.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/category/{id}")]
pub async fn category_item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match categories::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/item")]
pub async fn item_all(page: web::Query<Pageable>, pool: WDPool) -> HttpResponse {
    match items::find_all(page.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/item/{id}")]
pub async fn item(item_id: web::Path<u32>, pool: WDPool) -> HttpResponse {
    match items::find_by_id(item_id.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}