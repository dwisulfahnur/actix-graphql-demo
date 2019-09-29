use std::sync::Arc;

use juniper;
use juniper::{Executor, FieldResult, RootNode};
use juniper_from_schema::graphql_schema_from_file;
use serde::export::Default;

use crate::models::{Product, User};

graphql_schema_from_file!("src/schema.graphql");

pub struct Context;

impl juniper::Context for Context {}


pub struct Query;

impl QueryFields for Query {
    fn field_users(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
        id: Option<String>,
    ) -> FieldResult<Vec<User>> {
        let users: Vec<User> = Vec::new();
        Ok(users)
    }

    fn field_products(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Product, Walked>,
        id: Option<String>,
        user_id: Option<String>,
    ) -> FieldResult<Vec<Product>> {
        let products: Vec<Product> = vec![Product {
            id: match id {
                Some(_id) => _id,
                None => "".to_string()
            },
            user_id: match user_id {
                Some(_user_id) => _user_id,
                None => "".to_string()
            },
            name: "".to_string(),
            price: 0.0,
        }];
        Ok(products)
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_add_product(
        &self,
        executor: &juniper::Executor<'_, Context>,
        trail: &QueryTrail<'_, Product, Walked>,
        user_id: String,
        name: String,
        price: f64,
    ) -> FieldResult<Product> {
        // not implemented
        let mut product = Product::default();
        product.user_id = user_id;
        product.name = name;
        product.price = price;
        Ok(product)
    }

    fn field_add_user(
        &self,
        executor: &juniper::Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
        name: String,
        email: String,
    ) -> FieldResult<User> {

        // not implemented
        let mut user = User::default();
        user.name = name;
        user.email = email;
        Ok(user)
    }
}

impl ProductFields for Product {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.id)
    }
    fn field_user_id(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.user_id)
    }
    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
    fn field_price(&self, _: &Executor<'_, Context>) -> FieldResult<&f64> {
        Ok(&self.price)
    }
}

impl UserFields for User {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.id)
    }
    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
    fn field_email(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.email)
    }
}
