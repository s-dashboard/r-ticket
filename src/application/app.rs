use std::str::FromStr;
use warp::{filters::{path::FullPath, BoxedFilter}, redirect, reply::Reply, Filter, http::Uri};

pub async fn app() {

    let current_dir = std::env::current_dir()
        .expect("failed to read current directory");

    let wwwroot = current_dir.as_path().join("wwwroot").to_path_buf();

    let routes = root_redirect()
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
// fn create_response(status: u16, content_type: &str, body: &str) -> Result<Response<String>, warp::http::Error> {
//     warp::http::Response::builder()
//         .header("content-type", content_type.to_string())
//         .status(status)
//         .body(body.to_string())
// }

// fn create_redirect(status: u16, location: &str) -> Result<Response<String>, warp::http::Error> {
//     let final_status : u16;

//     match status {
//         301 | 302 | 303 | 307 | 308 => final_status = status,
//         _ => final_status = 302
//     }

//     warp::http::Response::builder()
//         .header("Location", location)
//         .status(final_status)
//         .body("".to_string())
// }
