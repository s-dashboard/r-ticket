use crate::store::tickets;
use warp::{Filter, Rejection, Reply};
use std::collections::HashMap;

use super::authentication;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    get_tickets().or(get_ticket())
}

fn get_tickets() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("tickets"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and(authentication::with_authentication())
        .and_then(tickets::tickets_list)
}

fn get_ticket() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path!("tickets" / i32))
        .and(warp::path::end())
        .and(authentication::with_authentication())
        .and_then(tickets::ticket_single)
}