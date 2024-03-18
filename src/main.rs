mod store;
mod routes;
mod db;
use crate::routes::app;

#[tokio::main]
async fn main() {
    app::app().await;
}