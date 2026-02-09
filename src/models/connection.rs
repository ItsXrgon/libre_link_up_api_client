use crate::models::common::{ActiveSensor, AuthTicket, Connection, GlucoseItem};
use serde::{Deserialize, Serialize};

/// Response from the connection endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub status: i32,
    pub data: ConnectionData,
    pub ticket: AuthTicket,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionData {
    pub connection: Connection,
    #[serde(rename = "activeSensors")]
    pub active_sensors: Vec<ActiveSensor>,
    #[serde(rename = "graphData")]
    pub graph_data: Vec<GlucoseItem>,
}
