use crate::models::common::{ActiveSensor, AuthTicket, Connection, GlucoseItem};
use serde::{Deserialize, Serialize};

/// Graph data containing connection, sensors, and historical glucose readings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphData {
    /// Connection information
    pub connection: Connection,
    /// Active sensors with device information
    #[serde(rename = "activeSensors")]
    pub active_sensors: Vec<ActiveSensor>,
    /// Historical glucose data for graph visualization
    #[serde(rename = "graphData")]
    pub graph_data: Vec<GlucoseItem>,
}

/// Response from the graph data endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphResponse {
    /// HTTP status code
    pub status: i32,
    /// Graph data containing connection, sensors, and historical glucose readings
    pub data: GraphData,
    /// Authentication ticket for future requests
    pub ticket: AuthTicket,
}
