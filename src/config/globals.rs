
use dotenv::dotenv;
use warp::reject::Rejection;
use std::env;

#[derive(Clone)]
pub struct GlobalSettings {
    pub connection_string: String,
    pub salt_and_pepper: String
}

pub fn init() -> GlobalSettings {
    dotenv().ok();
    let connection_string = env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");
    let salt_and_pepper_str = env::var("SALT_AND_PEPPER").expect("SALT_AND_PEPPER must be set");

    GlobalSettings {
        connection_string: connection_string,
        salt_and_pepper: salt_and_pepper_str
    }
}

pub fn with_settings() -> Result<GlobalSettings, Rejection>   {
    let settings = init();
    Ok(settings)
}