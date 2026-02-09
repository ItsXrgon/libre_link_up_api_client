use crate::models::common::{ActiveSensor, AuthTicket, Connection, GlucoseItem};
use serde::{Deserialize, Serialize};

/// Graph data containing connection, sensors, and historical glucose readings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphData {
    pub connection: Connection,
    #[serde(rename = "activeSensors")]
    pub active_sensors: Vec<ActiveSensor>,
    #[serde(rename = "graphData")]
    pub graph_data: Vec<GlucoseItem>,
}

/// Response from the graph data endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphResponse {
    pub status: i32,
    pub data: GraphData,
    pub ticket: AuthTicket,
}
