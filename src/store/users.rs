use serde::{Serialize, Deserialize};
use mysql::{params, Value};
use crate::db::sql;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>
}

impl User {}

pub type Users = Vec<User>;

pub async fn user_single(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let result = select_user(id);
    let ticket = result.unwrap();
    Ok(warp::reply::json(&ticket))
}

fn user_selector() -> impl Fn((Value, Value, Value)) -> User  {
    let selector = |(id, username, email)|
    User {
        id: mysql::from_value(id), 
        username: mysql::from_value(username), 
        email: mysql::from_value(email)
    };

    return selector;
}

fn select_user(id: i32) -> Option<User> {

    let query: &str = "SELECT 
        u.id
        , u.username
        , u.email
    FROM users u
    WHERE u.id = :id;";

    let result: Vec<User> = sql::select(query.to_string(), params! {"id" => &id },
        user_selector()
    ).unwrap();

    let first: Option<User> = result.into_iter().nth(0);
    return first;
}
