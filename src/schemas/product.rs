use juniper::{FieldError, FieldResult, RootNode};
use juniper;
use mysql::{Error as DBError, from_row, params, Row};

#[derive(Default, Debug, GraphQLObject)]
#[graphql(description = "Product")]
pub struct Product {
    id: String,
    user_id: String,
    name: String,
    price: f64,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Product Input")]
pub struct ProductInput {
    user_id: String,
    name: String,
    price: f64,
}