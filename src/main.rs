#[macro_use]
extern crate juniper;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_mysql;

use actix_web::{App, HttpServer, middleware, Responder, web};

use crate::handlers::{graphql, graphql_playground, register};
use crate::db::get_db_pool;

mod handlers;
mod schemas;
mod db;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();
    let pool = get_db_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(register)
            .default_service(web::to(|| "404"))
    })
        .bind("127.0.0.1:8080")?
        .run()
}