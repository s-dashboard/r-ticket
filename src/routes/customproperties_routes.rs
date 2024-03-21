// ?ownerType=tickets&id=4
use warp::{Filter, Rejection, Reply};
use crate::store::customproperties;
use super::authentication;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let property_path = warp::path("api")
            .and(warp::path("properties"));
        
    property_path
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::param())
        .and(authentication::with_authentication())
        .and_then(customproperties::property_list)
}