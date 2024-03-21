mod store;
mod routes;
mod db;
mod security;
mod config;
mod viewmodels;

use crate::routes::app;

#[tokio::main]
async fn main() {
    // let val = hasher::get_hashed("test01!".to_string()); 
    // println!("Hashed: {val}");
    app::app().await;
}