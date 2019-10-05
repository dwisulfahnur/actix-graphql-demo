use juniper::{FieldError, FieldResult, RootNode};
use juniper;
use mysql::{Error as DBError, from_row, params, Row};
use crate::schemas::root::Context;


#[derive(Default, Debug, GraphQLObject)]
#[graphql(description = "User")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
    pub name: String,
    pub email: String,
}
