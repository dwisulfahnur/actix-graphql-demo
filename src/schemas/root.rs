use juniper::{FieldError, FieldResult, RootNode};
use juniper;
use mysql::{Error as DBError, from_row, params, Row};

use crate::db::Pool;

use super::user::{User, UserInput};

pub struct Context {
    pub dbpool: Pool
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();
        let users = conn.prep_exec("select * from user", ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|mut row| {
                    let (id, name, email) = from_row(row);
                    User { id, name, email }
                }).collect()
            }).unwrap();
        Ok(users)
    }

    #[graphql(description = "Get Single user reference by user ID")]
    fn user(context: &Context, id: String) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();

        let user: Result<Option<Row>, DBError> = conn.first_exec(
            "SELECT * FROM user WHERE id=:id",
            params! {"id" => id},
        );

        if let Err(err) = user {
            return Err(FieldError::new(
                "User Not Found",
                graphql_value!({ "not_found": "user not found" }),
            ));
        }

        let (id, name, email) = from_row(user.unwrap().unwrap());
        Ok(User { id, name, email })
    }
}


pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_user(context: &Context, user: UserInput) -> FieldResult<User> {
        let mut conn = context.dbpool.get().unwrap();
        let new_id = uuid::Uuid::new_v4().to_simple().to_string();

        let insert: Result<Option<Row>, DBError> = conn.first_exec(
            "INSERT INTO user(id, name, email) VALUES(:id, :name, :email)",
            params! {
                "id" => &new_id.to_owned(),
                "name" => &user.name.to_owned(),
                "email" => &user.email.to_owned(),
            },
        );

        match insert {
            Ok(opt_row) => {
                Ok(User {
                    id: new_id,
                    name: user.name,
                    email: user.email,
                })
            }
            Err(err) => {
                let msg = match err {
                    DBError::MySqlError(err) => err.message,
                    _ => "internal error".to_owned()
                };
                Err(FieldError::new(
                    "Failed to create new user",
                    graphql_value!({ "internal_error": msg }),
                ))
            }
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
