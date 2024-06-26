use parking_lot::RwLock;
use std::sync::Arc;
use super::customproperties::CustomProperties;
use super::tickets::Tickets;
use super::clients::Clients;
use super::users::Users;

#[derive(Clone)]
pub struct Store {
    pub tickets_list: Arc<RwLock<Tickets>>,
    pub clients_list: Arc<RwLock<Clients>>,
    pub users_list: Arc<RwLock<Users>>,
    pub property_list: Arc<RwLock<CustomProperties>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            tickets_list: Arc::new(RwLock::new(Vec::new())),
            clients_list: Arc::new(RwLock::new(Vec::new())),
            users_list: Arc::new(RwLock::new(Vec::new())),
            property_list: Arc::new(RwLock::new(Vec::new()))
        }
    }
}