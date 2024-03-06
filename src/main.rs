mod routes;
mod tickets_store;
mod db;
use crate::routes::app;

#[tokio::main]
async fn main() {
    app::app().await;
}