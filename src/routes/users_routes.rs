use crate::store::users;
use warp::{Filter, Rejection, Reply};
use super::authentication;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    get_user()
    .or(post_user_auth())
}

fn get_user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {

    warp::get()
        .and(warp::path("api"))
        .and(warp::path!("users" / i32))
        .and(warp::path::end())
        .and(authentication::with_authentication())
        .and_then(users::user_single)
}

fn post_user_auth() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::post()
        .and(warp::path("api"))
        .and(warp::path("auth"))        
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and(warp::path::end())
        .and_then(users::user_auth)
}