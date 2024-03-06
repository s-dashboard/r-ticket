use crate::tickets_store::tickets;
use warp::{Filter, Rejection, Reply};
use std::collections::HashMap;

pub fn get_tickets() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path("tickets"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(tickets::tickets_list)
}