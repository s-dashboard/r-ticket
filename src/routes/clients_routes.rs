use crate::store::clients;
use warp::{Filter, Rejection, Reply};

use super::authentication;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy{
    
    let clients_path = warp::path("api")
            .and(warp::path("clients"));

    clients_path
        .and(warp::get())
        .and(warp::path::param())
        .and(authentication::with_authentication())
        .and_then(clients::client_single)        
}