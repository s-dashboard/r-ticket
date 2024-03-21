use std::{convert::Infallible, str::FromStr};
use serde::Serialize;
use warp::{filters::{path::FullPath, BoxedFilter}, http::Uri, redirect, reject::Rejection, reply::Reply, Filter};
use crate::routes::tickets_routes;
use crate::routes::users_routes;
use crate::routes::clients_routes;

use super::authentication::Unauthorized;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String
}

pub async fn app() {

    let current_dir = std::env::current_dir()
        .expect("failed to read current directory");

    let wwwroot = current_dir.as_path().join("wwwroot").to_path_buf();

    let routes = root_redirect()
        .or(tickets_routes::routes())
        .or(clients_routes::routes())
        .or(users_routes::post_user_auth())
        .or(users_routes::routes())
        .or(warp::fs::dir(wwwroot))
        .recover(handle_rejection);
        
    // Server the filter
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(Unauthorized) = err.find() {
        code = warp::http::StatusCode::UNAUTHORIZED;
        message = "Invalid authorization";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
       // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

fn root_redirect() -> BoxedFilter<(impl Reply,)> {
    warp::path::full()
        .and_then(move |path: FullPath| async move {
            let path = path.as_str();

            // do not redirect if the path ends in a trailing slash
            // or contains a period (indicating a specific file, e.g. style.css)
            if path.ends_with("/") || path.contains(".") {
                return Err(warp::reject());
            }

            Ok(redirect::redirect(
                Uri::from_str(&[path, "/"].concat()).unwrap(),
            ))
        })
        .boxed()
}