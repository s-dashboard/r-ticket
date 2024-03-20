use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use mysql::{params, Value};
use crate::{db::sql, routes::authentication::{self}, security::hasher};

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>
}

impl User {}

pub type Users = Vec<User>;

pub async fn user_single(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let result = select_user(id);
    let user = result.unwrap();
    Ok(warp::reply::json(&user))
}

pub async fn user_auth(map: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {

    let username: String = map.get("username")
        .unwrap().to_string();

    let password: String = map.get("password")
        .unwrap().to_string();

    let user: User = find_user_by_username(username)
        .expect("Wrong username or password");

    if user.password.is_some() {
        let hash = user.password.unwrap();

        hasher::verify_hash(password, hash)
        .expect("Wrong username or password");
        
        let token = authentication::create_token(user.id);
        if token.is_some() {
            return Ok(warp::reply::with_status(warp::reply::json(&token),warp::http::StatusCode::OK)) 
        }
    }

    return Ok(warp::reply::with_status(warp::reply::json(&"Unauthorized"),warp::http::StatusCode::UNAUTHORIZED));
}

fn user_selector() -> impl Fn((Value, Value ,Value, Value)) -> User  {
    let selector = |(id, username, email, password)|
    User {
        id: mysql::from_value(id), 
        username: mysql::from_value(username), 
        email: mysql::from_value(email),
        password: mysql::from_value(password)
    };

    return selector;
}

fn find_user_by_username(username: String) -> Option<User> {
    let query: &str = "SELECT 
        u.id
        , u.username
        , u.email
        , u.password
    FROM users u
    WHERE u.username = :username;";

    let result: Vec<User> = sql::select(query.to_string(), params! {"username" => &username },
        user_selector()
    ).unwrap();

    let first: Option<User> = result.into_iter().nth(0);
    return first;
}

fn select_user(id: i32) -> Option<User> {

    let query: &str = "SELECT 
        u.id
        , u.username
        , u.email
        , '' as password
    FROM users u
    WHERE u.id = :id;";

    let result: Vec<User> = sql::select(query.to_string(), params! {"id" => &id },
        user_selector()
    ).unwrap();

    let first: Option<User> = result.into_iter().nth(0);
    return first;
}
