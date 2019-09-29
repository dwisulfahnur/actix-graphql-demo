use std::sync::Arc;

use actix_web::{Error, HttpResponse, web};
use futures::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::graphql::{Context, Mutation, Query, Schema};

pub fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    let ctx = Context {};
    web::block(move || {
        let res = data.execute(&schema, &ctx);
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
        .map_err(Error::from)
        .and_then(|res| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(res))
        })
}

pub fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql/"))
}


pub fn register(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(Schema::new(Query, Mutation));

    config
        .data(schema)
        .route("/graphql", web::post().to_async(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
}