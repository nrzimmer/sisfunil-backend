extern crate actix_web;
extern crate diesel;

use std::{env, io};
use actix_web::{App, HttpServer, middleware};
use crate::database::mysql::get_pool;

mod models {
    pub mod group;
    pub mod itembox;
    pub mod itemtype;
    pub mod location;
    pub mod category;
    pub mod item;
}

mod web {
    pub mod router;
    pub mod types;
    pub mod error;
}

mod repositories {
    pub mod location;
}

mod database {
    pub mod mysql;
    pub mod schema;
    pub mod types;
}
mod constants;
mod response;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(get_pool().clone())
            .wrap(middleware::Logger::default())
            //.data(database::mysql::get_pool().clone())
            .service(web::router::location_list)
            .service(web::router::location_item)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
