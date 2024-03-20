use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use mysql::{params, Value};
use serde::{Deserialize, Serialize};
use warp::{filters::header, reject::Rejection, reply::{self, Reply, Response, WithStatus}, Filter};
use crate::db::sql;

#[derive(Copy, Clone)]
pub struct UserContext {
    pub user_id: i32
}

fn verify_token(token: String) -> Result<i32, i32> {

    let select_sql = String::from("SELECT user_id, token_value, UNIX_TIMESTAMP(valid_to) as valid_to_nix FROM tokens WHERE token_value = :token_value;");
    let created_tokens = sql::select(select_sql, params! {"token_value" => &token },
        token_selector()
    ).unwrap();

    if created_tokens.is_empty() {
        return Err(0);
    }

    let created: Token = created_tokens.into_iter().nth(0).unwrap();
    let user_id = created.user_id.unwrap(); 
    
    return Ok(user_id)
}

pub async fn api_token_filter() -> impl Filter<Extract = (WithStatus<impl Reply>,), Error = Rejection> {
    warp::header::header("Authorization")
        .and_then(authorize_token)
        .boxed()
}

async fn authorize_token(token: String) ->  Result<WithStatus<impl Reply>, Rejection> {
    let token_id = match verify_token(token) {
        Ok(t) => t, 
        Err(_) => 0
    };

    if token_id == 0 {
        return Ok(warp::reply::with_status(warp::reply(), warp::http::StatusCode::UNAUTHORIZED))
    }

    return Ok(warp::reply::with_status(warp::reply(), warp::http::StatusCode::OK))
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_value: String, 
    pub user_id: Option<i32>,
    pub valid_to: Option<DateTime<Utc>>
}

fn token_value() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(126)
        .map(char::from)
        .collect();

    return  random_string; 
}

pub fn create_token(user_id: i32) -> Option<Token> {

    let token_value_str: String = token_value();
    let tokens: Vec<Token> = vec![
        Token {
            token_value: String::from(token_value_str),
            user_id: Some(user_id),
            valid_to: None   
        }
    ];

    let delete_tokens= String::from("DELETE FROM tokens WHERE user_id = :user_id;");
    let delete_result = sql::execute(delete_tokens, tokens.iter().map(|p: &Token| params! {
        "user_id" => p.user_id
    }));

    if delete_result.is_ok() {
        let insert_token = String::from("INSERT INTO tokens(user_id, token_value, valid_to) 
            VALUES(:user_id, :token_value, DATE_ADD(utc_timestamp(), INTERVAL 2 HOUR)
        );");

        let result = sql::execute(insert_token, tokens.iter().map(|p: &Token| params! {
            "user_id" => p.user_id, 
            "token_value" => p.token_value.to_string()
        }));
    
        if result.is_ok() {

            let select_sql = String::from("SELECT user_id, token_value, UNIX_TIMESTAMP(valid_to) as valid_to_nix FROM tokens WHERE user_id = :user_id;");
            let created_tokens = sql::select(select_sql, params! {"user_id" => &user_id },
                token_selector()
            ).unwrap();

            let created: Option<Token> = created_tokens.into_iter().nth(0);
            return created;
        }
    }

    return None;
}

fn token_selector() -> impl Fn((Value, Value, Value)) -> Token  {
    let selector = |(user_id, token_value, valid_to_nix)|
    Token {
        user_id: mysql::from_value(user_id), 
        token_value: mysql::from_value(token_value), 
        valid_to: DateTime::from_timestamp(mysql::from_value(valid_to_nix),0),
    };
    return selector;
}