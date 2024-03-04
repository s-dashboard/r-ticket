use parking_lot::RwLock;
use std::sync::Arc;
use std::vec::Vec;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub type Tickets = Vec<Ticket>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Ticket {
    id: i32,
    subject: String,
    created: DateTime<Utc>
}

impl Ticket {
    pub fn new(id: i32, subject: String, created: DateTime<Utc>) -> Self {
        Ticket {
            id: id, 
            subject: subject,
            created: created
        }
    }
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

pub async fn tickets_list() -> Result<impl warp::Reply, warp::Rejection> {    
    let store = super::tickets::Store::new();
    let dt = Utc::now();
    store.tickets_list.write().push(Ticket::new(1, "testar".to_string(), dt));
    store.tickets_list.write().push(Ticket::new(2, "testar".to_string(), dt));
    store.tickets_list.write().push(Ticket::new(3, "testar".to_string(), dt));
    store.tickets_list.write().push(Ticket::new(4, "testar".to_string(), dt));
    
    let result = store.tickets_list.read();
    Ok(warp::reply::json(&*result))
}