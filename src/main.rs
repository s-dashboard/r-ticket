mod store;
mod routes;
mod db;
mod security;

use crate::routes::app;

#[tokio::main]
async fn main() {
    // let val = hasher::get_hashed("test01!".to_string()); 
    // println!("Hashed: {val}");
    app::app().await;
}