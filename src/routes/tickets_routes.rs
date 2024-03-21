use crate::{store::tickets, viewmodels::models};
use warp::{Filter, Rejection, Reply};
use std::collections::HashMap;
use super::authentication;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    put_ticket().or(get_tickets()).or(get_ticket())
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

fn put_ticket() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::put()
        .and(json_body())
        .and(warp::path("api"))
        .and(warp::path!("tickets" / i32))
        .and(warp::path::end())
        .and(authentication::with_authentication())
        .and_then(tickets::save)
}

fn json_body() -> impl Filter<Extract = (models::TicketModel,), Error = warp::Rejection> + Clone{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json::<models::TicketModel>())
}