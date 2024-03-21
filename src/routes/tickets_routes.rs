use crate::{store::tickets, viewmodels::models::TicketModel};
use warp::{Filter, Rejection, Reply};
use std::collections::HashMap;
use super::{authentication, helpers};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let tickets_path = warp::path("api")
            .and(warp::path("tickets"));

    tickets_path
        .and(warp::get())
        .and(warp::path::param())
        .and(authentication::with_authentication())
        .and_then(tickets::ticket_single)
        .or(tickets_path
            .and(warp::get())
            .and(warp::query::<HashMap<String, String>>())
            .and(authentication::with_authentication())
            .and_then(tickets::tickets_list))
        .or(tickets_path
            .and(warp::post())
            .and(helpers::json_body::<TicketModel>())
            .and(authentication::with_authentication())
            .and_then(tickets::insert))
        .or(tickets_path
            .and(warp::put())
            .and(helpers::json_body::<TicketModel>())
            .and(warp::path::param())
            .and(authentication::with_authentication())
            .and_then(tickets::update))
        .or(tickets_path
            .and(warp::delete())
            .and(warp::path::param())
            .and(authentication::with_authentication())
            .and_then(tickets::delete)
        )

}