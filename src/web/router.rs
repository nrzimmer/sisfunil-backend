use actix_web::{get, web, HttpResponse};
use serde::Serialize;

use crate::constants::APPLICATION_JSON;
use crate::repositories::{categories, containers, groups, items, kinds, locations};
use crate::web::error::diesel_error;
use crate::web::filter::{FilterConfig, ToFilter};
use crate::web::pageable::Pageable;

use super::types::WDPool;

fn http_ok_json<T: Serialize>(json: T) -> HttpResponse {
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(json)
}

#[get("/location")]
pub async fn location_list(page: web::Query<Pageable>, pool: WDPool) -> HttpResponse {
    match locations::find_all(page.into_inner(), &pool) {
        Ok(v) => http_ok_json(v),
        Err(e) => diesel_error(e),
    }
}

#[get("/location/search")]
pub async fn location_search(
    words: web::Query<FilterConfig>,
    page: web::Query<Pageable>,
    pool: WDPool,
) -> HttpResponse {
    match locations::search(words.into_inner().to_filter(), page.into_inner(), &pool) {
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

#[get("/group/search")]
pub async fn group_search(
    words: web::Query<FilterConfig>,
    page: web::Query<Pageable>,
    pool: WDPool,
) -> HttpResponse {
    match groups::search(words.into_inner().to_filter(), page.into_inner(), &pool) {
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

#[get("/container/search")]
pub async fn container_search(
    words: web::Query<FilterConfig>,
    page: web::Query<Pageable>,
    pool: WDPool,
) -> HttpResponse {
    match containers::search(words.into_inner().to_filter(), page.into_inner(), &pool) {
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

#[get("/kind/search")]
pub async fn kind_search(
    words: web::Query<FilterConfig>,
    page: web::Query<Pageable>,
    pool: WDPool,
) -> HttpResponse {
    match kinds::search(words.into_inner().to_filter(), page.into_inner(), &pool) {
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

#[get("/category/search")]
pub async fn category_search(
    words: web::Query<FilterConfig>,
    page: web::Query<Pageable>,
    pool: WDPool,
) -> HttpResponse {
    match categories::search(words.into_inner().to_filter(), page.into_inner(), &pool) {
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

#[get("/item/search")]
pub async fn item_search(
    words: web::Query<FilterConfig>,
    page: web::Query<Pageable>,
    pool: WDPool,
) -> HttpResponse {
    match items::search(words.into_inner().to_filter(), page.into_inner(), &pool) {
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
