use crate::store::clients;
use warp::{Filter, Rejection, Reply};

use super::authentication;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy{
    get_client()
}

fn get_client() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy{
    warp::get()
        .and(warp::path("api"))
        .and(warp::path!("clients" / i32))
        .and(warp::path::end())
        .and(authentication::with_authentication())
        .and_then(clients::client_single)        
}