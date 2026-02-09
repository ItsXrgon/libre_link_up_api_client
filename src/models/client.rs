use crate::models::common::{ActiveSensor, Connection, GlucoseItem};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trend direction for glucose readings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendType {
    SingleDown,
    FortyFiveDown,
    Flat,
    FortyFiveUp,
    SingleUp,
    NotComputable,
}

/// Processed glucose data for consumption
///
/// # Examples
///
/// ```
/// use libre_link_up_api_client::{LibreCgmData, TrendType};
/// use chrono::Utc;
///
/// let data = LibreCgmData {
///     value: 120.0,
///     is_high: false,
///     is_low: false,
///     trend: TrendType::Flat,
///     date: Utc::now(),
/// };
/// assert_eq!(data.value, 120.0);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LibreCgmData {
    /// Glucose value in mg/dL
    pub value: f64,
    /// Whether the value is above the target high
    #[serde(rename = "isHigh")]
    pub is_high: bool,
    /// Whether the value is below the target low
    #[serde(rename = "isLow")]
    pub is_low: bool,
    /// Trend direction
    pub trend: TrendType,
    /// Timestamp of the reading
    pub date: DateTime<Utc>,
}

/// Response from the read() method containing current and historical glucose data
///
/// # Examples
///
/// ```no_run
/// use libre_link_up_api_client::LibreLinkUpClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = LibreLinkUpClient::simple(
///     "email@example.com".to_string(),
///     "password".to_string(),
///     None,
/// )?;
///
/// let response = client.read().await?;
/// println!("Current: {:.1} mg/dL", response.current.value);
/// println!("History: {} readings", response.history.len());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadResponse {
    /// Current glucose reading
    pub current: LibreCgmData,
    /// Historical glucose readings
    pub history: Vec<LibreCgmData>,
}

/// Response from the read_raw() method with unparsed API data
///
/// Access to raw API responses for advanced use cases
///
/// # Examples
///
/// ```no_run
/// use libre_link_up_api_client::LibreLinkUpClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = LibreLinkUpClient::simple(
///     "email@example.com".to_string(),
///     "password".to_string(),
///     None,
/// )?;
///
/// let raw = client.read_raw().await?;
/// println!("Connection ID: {}", raw.connection.patient_id);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadRawResponse {
    /// Connection information
    pub connection: Connection,
    /// Active sensors
    pub active_sensors: Vec<ActiveSensor>,
    /// Graph data (historical glucose readings)
    pub graph_data: Vec<GlucoseItem>,
}
