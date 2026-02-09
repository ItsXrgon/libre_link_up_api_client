use crate::models::common::{AuthTicket, Connection};
use serde::{Deserialize, Serialize};

/// Response containing a list of connections and authentication ticket
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    /// HTTP status code
    pub status: i32,
    /// List of connections
    pub data: Vec<Connection>,
    /// Authentication ticket for future requests
    pub ticket: AuthTicket,
}
