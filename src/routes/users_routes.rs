use crate::store::users;
use warp::{Filter, Rejection, Reply};

pub fn get_user() -> impl Filter<Extract = impl Reply, Error = Rejection> + Copy {
    warp::get()
        .and(warp::path("api"))
        .and(warp::path!("users" / i32))
        .and(warp::path::end())
        .and_then(users::user_single)
}