use chrono::NaiveDateTime;
use mysql::{params, Value};
use parking_lot::RwLock;
use std::sync::Arc;
use std::vec::Vec;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::db::sql;

pub type Tickets = Vec<Ticket>;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Ticket {
    pub id: i32,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub state: Option<String>,
    pub state_title: Option<String>,
    pub created: Option<NaiveDateTime>,
    pub changed: Option<NaiveDateTime>
}

impl Ticket {
    // pub fn new(id: i32, subject: Option<String>, state: Option<String>) -> Self {
    //     Ticket {
    //         id: id, 
    //         subject: subject,
    //         state: state
    //     }
    // }
}

#[derive(Clone)]
pub struct Store {
    tickets_list: Arc<RwLock<Tickets>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            tickets_list: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

fn get_state(mut _query: HashMap<String, String>) -> String  {
    let mut state: &str = &"new";
    
    if _query.contains_key("state") {
        state = _query.get_mut("state").unwrap();
    }

    return state.to_string();
}

pub async fn tickets_list(mut _query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {    
    let store: Store = super::tickets::Store::new();
    let state: String = get_state(_query);
    let result: Vec<Ticket> = select_tickets(state);

    result.iter().for_each(|item|{
        store.tickets_list.write().push(item.clone())        
    });

    let store_result = store.tickets_list.read();
    
    Ok(warp::reply::json(&*store_result))

}

pub async fn ticket_single(id: i32) -> Result<impl warp::Reply, warp::Rejection> {    
    let result = select_ticket(id);
    let ticket = result.unwrap();
    Ok(warp::reply::json(&ticket))
}

fn ticket_selector() -> impl Fn((Value, Value, Value, Value, Value, Value, Value)) -> Ticket
{
    let selector = |(id, subject, content, state, state_title, created_nix, changed_nix)|
    Ticket {
        id: mysql::from_value(id), 
        subject: mysql::from_value(subject), 
        content: mysql::from_value(content), 
        state: mysql::from_value(state),
        state_title: mysql::from_value(state_title),
        created: NaiveDateTime::from_timestamp_opt(mysql::from_value::<i64>(created_nix),0),
        changed: NaiveDateTime::from_timestamp_opt(mysql::from_value(changed_nix),0)
    };
    return selector;
}

fn select_tickets(state: String) -> Vec<Ticket> {

    let query: &str = "SELECT 
        t.id
        , subject
        , ticket as content
        , state
        , s.title AS state_title 
        , UNIX_TIMESTAMP(created) as created_nix
        , UNIX_TIMESTAMP(changed) as changed_nix
    FROM tickets t JOIN states s ON s.id = t.state WHERE t.state = :state;";

    let result: Vec<Ticket> = sql::select(query.to_string(), params! {"state" => &state },
        ticket_selector()
    ).unwrap();

    return result;
}

fn select_ticket(id: i32) -> Option<Ticket> {

    let query: &str = "SELECT 
        t.id
        , subject
        , ticket as content
        , state
        , s.title AS state_title 
        , UNIX_TIMESTAMP(created) as created_nix
        , UNIX_TIMESTAMP(changed) as changed_nix
    FROM tickets t JOIN states s ON s.id = t.state WHERE t.id = :id;";

    let result: Vec<Ticket> = sql::select(query.to_string(), params! {"id" => &id },
        ticket_selector()
    ).unwrap();

    let first = result.into_iter().nth(0);
    return first;
}