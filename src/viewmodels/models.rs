use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketModel {
    pub id: i32,
    pub client_id: i32,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub state: Option<String>
}

