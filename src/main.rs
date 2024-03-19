mod store;
mod routes;
mod db;
mod security;
use crate::routes::app;

#[tokio::main]
async fn main() {
    app::app().await;
}