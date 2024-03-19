use serde::{Serialize, Deserialize};
// use mysql::{params, Value};
// use crate::db::sql;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>
}

impl User {}

pub type Users = Vec<User>;