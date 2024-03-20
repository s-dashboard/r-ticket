use std::str::FromStr;
use warp::{filters::{path::FullPath, BoxedFilter}, redirect, reply::Reply, Filter, http::Uri};
use crate::routes::tickets_routes;
use crate::routes::users_routes;
use crate::routes::clients_routes;

// use super::authentication;

pub async fn app() {

    let current_dir = std::env::current_dir()
        .expect("failed to read current directory");

    let wwwroot = current_dir.as_path().join("wwwroot").to_path_buf();
    
    // let filter = authentication::api_token_filter();

    let routes = root_redirect()
        .or(tickets_routes::get_tickets())
        .or(tickets_routes::get_ticket())
        .or(clients_routes::get_client())
        .or(users_routes::get_user())
        .or(users_routes::post_user_auth())
        .or(warp::fs::dir(wwwroot));
        
    // Server the filter
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
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