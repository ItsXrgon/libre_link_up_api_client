use crate::models::common::{ActiveSensor, AuthTicket, Connection, GlucoseItem};
use serde::{Deserialize, Serialize};

/// Response containing connection details and sensor information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionResponse {
    /// HTTP status code
    pub status: i32,
    /// Connection details and sensor information
    pub data: ConnectionData,
    /// Authentication ticket for future requests
    pub ticket: AuthTicket,
}

/// Detailed connection data including active sensors and glucose graph
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConnectionData {
    pub connection: Connection,
    #[serde(rename = "activeSensors")]
    pub active_sensors: Vec<ActiveSensor>,
    /// Historical glucose data for graph visualization
    #[serde(rename = "graphData")]
    pub graph_data: Vec<GlucoseItem>,
}
