use crate::models::common::{AuthTicket, Connection};
use serde::{Deserialize, Serialize};

/// Response from the connections list endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    pub status: i32,
    pub data: Vec<Connection>,
    pub ticket: AuthTicket,
}
