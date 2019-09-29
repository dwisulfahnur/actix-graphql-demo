use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub price: f64,
}
