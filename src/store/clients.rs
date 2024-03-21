use serde::{Serialize, Deserialize};
use mysql::{params, Value};
use crate::{db::sql, routes::authentication::UserContext};

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Client {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>
}

impl Client {}

pub type Clients = Vec<Client>;

fn client_selector() -> impl Fn((Value, Value, Value)) -> Client 
{
    let selector = |(id, name, email)|
    Client {
        id: mysql::from_value(id), 
        name: mysql::from_value(name), 
        email: mysql::from_value(email)
    };
    return selector;
}

pub async fn client_single(id: i32, _context: UserContext) -> Result<impl warp::Reply, warp::Rejection> {
    let result = select_client(id);
    let ticket = result.unwrap();
    Ok(warp::reply::json(&ticket))
}

fn select_client(id: i32) -> Option<Client> {

    let query: &str = "SELECT 
        c.id
        , c.name
        , c.email
    FROM clients c
    WHERE c.id = :id;";

    let result: Vec<Client> = sql::select(query.to_string(), params! {"id" => &id },
        client_selector()
    ).unwrap();

    let first = result.into_iter().nth(0);
    return first;
}


