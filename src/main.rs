#[macro_use]
extern crate actix_web;
use std::{env, io};
use actix_web::{App, HttpServer, middleware};

mod models {
    pub mod group;
    pub mod itembox;
    pub mod itemtype;
    pub mod location;
    pub mod category;
    pub mod item;
}

mod web {
    pub mod items;
}

mod database {
    pub mod mysql;
    pub mod schema;
}
mod constants;
mod response;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(database::mysql::get_pool().clone())
            .service(web::items::list)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
