mod application;
use application::app;

#[tokio::main]
async fn main() {
    app::app().await; 
}