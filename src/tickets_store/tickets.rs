use chrono::NaiveDateTime;
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

pub async fn tickets_list(mut _query: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {    
    let store: Store = super::tickets::Store::new();
    let state: String = get_state(_query);
    let result: Vec<Ticket> = sql::select_tickets(state)
        .unwrap();

    result.iter().for_each(|item|{
        store.tickets_list.write().push(item.clone())        
    });

    let store_result = store.tickets_list.read();
    
    Ok(warp::reply::json(&*store_result))
}

fn get_state(mut _query: HashMap<String, String>) -> String  {
    let mut state: &str = &"new";
    
    if _query.contains_key("state") {
        state = _query.get_mut("state").unwrap();
    }

    return state.to_string();
}