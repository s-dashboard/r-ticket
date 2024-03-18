use parking_lot::RwLock;
use std::sync::Arc;
use super::tickets::Tickets;
use super::clients::Clients;

#[derive(Clone)]
pub struct Store {
    pub tickets_list: Arc<RwLock<Tickets>>,
    pub clients_list: Arc<RwLock<Clients>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            tickets_list: Arc::new(RwLock::new(Vec::new())),
            clients_list: Arc::new(RwLock::new(Vec::new())),
        }
    }
}