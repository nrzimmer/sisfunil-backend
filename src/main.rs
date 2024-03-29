extern crate actix_web;
extern crate diesel;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

use crate::database::mysql::get_pool;

mod models {
    pub mod category;
    pub mod container;
    pub mod group;
    pub mod item;
    pub mod kind;
    pub mod location;
}

mod web {
    pub mod error;
    pub mod filter;
    pub mod pageable;
    pub mod router;
    pub mod types;
}

mod repositories {
    pub mod categories;
    pub mod containers;
    pub mod groups;
    pub mod items;
    pub mod kinds;
    pub mod locations;
}

mod dto {}

mod database {
    pub mod mysql;
    pub mod schema;
    pub mod types;
}

mod constants;
pub mod dtos;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(get_pool().clone())
            .wrap(middleware::Logger::default())
            .service(web::router::location_list)
            .service(web::router::location_search)
            .service(web::router::location_item)
            .service(web::router::container_list)
            .service(web::router::container_search)
            .service(web::router::container_item)
            .service(web::router::group_list)
            .service(web::router::group_search)
            .service(web::router::group_item)
            .service(web::router::kind_item)
            .service(web::router::kind_search)
            .service(web::router::kind_list)
            .service(web::router::category_item)
            .service(web::router::category_search)
            .service(web::router::category_list)
            .service(web::router::item_all)
            .service(web::router::item_search)
            .service(web::router::item)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
