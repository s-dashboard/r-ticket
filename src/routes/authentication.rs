use chrono::{DateTime, Utc};
use rand::{distributions::Alphanumeric, Rng};
use mysql::{params, Value};
use serde::{Deserialize, Serialize};
use warp::{reject::{self, Rejection}, Filter};
use crate::db::sql;

#[derive(Clone)]
pub struct UserContext {
    pub user_id: i32,
    pub username: String
}

#[derive(Debug, Copy, Clone)]
pub struct Unauthorized;
impl reject::Reject for Unauthorized {}

fn verify_token(token: String) -> Result<VerifiedToken, Rejection> {

    let mut fixed_token = token.replace("bearer ", "");
    fixed_token = fixed_token.replace("Bearer ", "");

    let select_sql = String::from("SELECT user_id, username FROM tokens INNER JOIN users ON users.id = tokens.user_id WHERE token_value = :token_value;");
    let created_tokens = sql::select(select_sql, params! {"token_value" => &fixed_token },
        verified_token_selector()
    ).unwrap();

    if created_tokens.is_empty() {
        Err(reject::custom(Unauthorized))
    } else {
        let created: VerifiedToken = created_tokens.into_iter().nth(0).unwrap();
        Ok(created)    
    }
}

pub fn with_authentication() ->  impl Filter<Extract = (UserContext,), Error = Rejection> + Copy {
    warp::header::<String>("Authorization").and_then(move |token: String| async move {
        match verify_token(token) {
            Ok(t) => {
                Ok(UserContext {
                    user_id: t.user_id.unwrap(),
                    username: t.username.unwrap()
                })
            },
            Err(_) => Err(reject::custom(Unauthorized))
        }
    })
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_value: String, 
    pub user_id: Option<i32>,
    pub valid_to: Option<DateTime<Utc>>
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct VerifiedToken {
    pub user_id: Option<i32>,
    pub username: Option<String>
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

fn verified_token_selector() -> impl Fn((Value, Value)) -> VerifiedToken  {
    let selector = |(user_id, username)|
    VerifiedToken {
        user_id: mysql::from_value(user_id),
        username: mysql::from_value(username),
    };
    return selector;
}