use crate::store::users;
use warp::{Filter, Rejection, Reply};
use super::{authentication, helpers};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    let user_path = warp::path("api")
        .and(warp::path("users")); 

    user_path
        .and(warp::get())
        .and(warp::path::param())
        .and(authentication::with_authentication())
        .and_then(users::user_single)
}

pub fn post_user_auth() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(helpers::json_body())
        .and(warp::path("api"))
        .and(warp::path("auth"))        
        .and(warp::path::end())
        .and_then(users::user_auth)
}