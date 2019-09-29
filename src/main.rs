#[macro_use]
extern crate juniper;
extern crate serde_json;

use actix_web::{App, HttpServer, middleware, Responder, web};

use graphql::Schema;

use crate::handlers::{graphql, graphql_playground, register};

mod graphql;
mod handlers;
mod models;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(register)
    })
        .bind("127.0.0.1:8080")?
        .run()
}