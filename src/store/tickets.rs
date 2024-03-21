use chrono::prelude::*;
use mysql::{params, Value};
use std::vec::Vec;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::{db::sql, routes::authentication::UserContext, viewmodels::models};
use super::store::Store;

pub type Tickets = Vec<Ticket>;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Ticket {
    pub id: i32,
    pub client_id: i32,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub state: Option<String>,
    pub state_title: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub changed: Option<DateTime<Utc>>
}

impl Ticket {}

fn get_state(mut _query: &HashMap<String, String>) -> String  {
    let mut state: &str = &"new";
    
    if _query.contains_key("state") {
        state = _query.get("state").unwrap();
    }

    return state.to_string();
}

fn get_search(mut _query: &HashMap<String, String>) -> String  {
    let mut state: &str = &"";
    
    if _query.contains_key("search") {
        state = _query.get("search").unwrap();
    }

    return state.to_string();
}

pub async fn tickets_list(mut _query: HashMap<String, String>, _context: UserContext) -> Result<impl warp::Reply, warp::Rejection> {    
    let store: Store = super::tickets::Store::new();
    let state: String = get_state(&_query);
    let search: String = get_search(&_query);
    let result: Vec<Ticket> = select_tickets(state, search);

    result.iter().for_each(|item|{
        store.tickets_list.write().push(item.clone())        
    });

    let store_result = store.tickets_list.read();
    Ok(warp::reply::json(&*store_result))
}

pub async fn ticket_single(id: i32, _context: UserContext) -> Result<impl warp::Reply, warp::Rejection> {    
    let result = select_ticket(id);
    let ticket = result.unwrap();
    Ok(warp::reply::json(&ticket))
}

fn ticket_selector() -> impl Fn((Value, Value, Value, Value, Value, Value, Value, Value)) -> Ticket
{
    let selector = |(id, client_id, subject, content, state, state_title, created_nix, changed_nix)|
    Ticket {
        id: mysql::from_value(id), 
        client_id: mysql::from_value(client_id), 
        subject: mysql::from_value(subject), 
        content: mysql::from_value(content), 
        state: mysql::from_value(state),
        state_title: mysql::from_value(state_title),
        created: DateTime::from_timestamp(mysql::from_value::<i64>(created_nix),0),
        changed: DateTime::from_timestamp(mysql::from_value(changed_nix),0)
    };
    return selector;
}

fn select_tickets(state: String, search: String) -> Vec<Ticket> {

    let mut query: String = "SELECT 
        t.id
        , client_id
        , subject
        , ticket as content
        , state
        , s.title AS state_title 
        , UNIX_TIMESTAMP(created) as created_nix
        , UNIX_TIMESTAMP(changed) as changed_nix
    FROM tickets t 
        JOIN states s ON s.id = t.state 
        JOIN clients c ON c.id = t.client_id 
    WHERE t.state = :state;".to_owned();

    let mut params: Vec<(String, Value)> = Vec::new();
    params.push(("state".to_string(), Value::from(&state)));

    if !search.is_empty() {
        let search_words: Vec<&str> = search.split(" ")
            .collect();

        let mut i: i32 = 0;
        for word in search_words {
            let word_p = format!("word_{0}", i);
            let word_sql = format!(" AND (subject LIKE '%' + {0} +'%')", word_p);
            query.push_str(&word_sql);
            params.push((word_p, Value::from(word.to_string())));
            i = i + 1;
        }
    }

    let result: Vec<Ticket> = sql::select(query.to_string(), params,
        ticket_selector()
    ).unwrap();

    return result;
}

fn select_ticket(id: i32) -> Option<Ticket> {

    let query: &str = "SELECT 
        t.id
        , client_id
        , subject
        , ticket as content
        , state
        , s.title AS state_title 
        , UNIX_TIMESTAMP(created) as created_nix
        , UNIX_TIMESTAMP(changed) as changed_nix
    FROM tickets t 
        JOIN states s ON s.id = t.state 
        JOIN clients c ON c.id = t.client_id 
    WHERE t.id = :id;";

    let result: Vec<Ticket> = sql::select(query.to_string(), params! {"id" => &id },
        ticket_selector()
    ).unwrap();

    let first = result.into_iter().nth(0);
    return first;
}

pub async fn save(ticket: models::TicketModel, id: i32, _context: UserContext) -> Result<impl warp::Reply, warp::Rejection> {
    let result = upsert_ticket(ticket, id);
    Ok(warp::reply::json(&result))
}

fn upsert_ticket(ticket: models::TicketModel, id: i32) -> Option<Ticket> {
    let existing_ticket: Option<Ticket> = select_ticket(id); 
    let tickets: Vec<models::TicketModel> = vec![ticket];

    if existing_ticket != None {
        let update_sql = String::from("UPDATE tickets SET subject = :subject, content = :content, client_id = :client_id, state = :state, changed = Now() WHERE id = :id");
        
        let result = sql::execute(update_sql, tickets.iter().map(|p: &models::TicketModel| params! {
            "subject" => p.subject.clone(), 
            "content" => p.content.clone(),
            "client_id" => p.client_id,
            "state" => p.state.clone(),
            "id" => p.id
        })); 

        if result.is_ok() {
            return select_ticket(existing_ticket.unwrap().id);
        } else {
            None
        }
    } else {
        let insert_sql = String::from("INSERT INTO tickets(subject, content, client_id, state, created, changed) VALUES(:subject, :content, :client_id, :state, Now(), Now())");
        let result = sql::execute_with_id(insert_sql, tickets.iter().map(|p: &models::TicketModel| params! {
            "subject" => p.subject.clone(), 
            "content" => p.content.clone(),
            "state" => p.state.clone(),
            "client_id" => p.client_id
        })); 

        if result.is_ok() {
            return select_ticket(result.unwrap()); 
        } else{
            return None; 
        }

    }
}