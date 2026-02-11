//! Logbook types for GET /llu/connections/{patientId}/logbook.

use crate::models::common::AuthTicket;
use serde::{Deserialize, Serialize};

/// Single logbook entry (glucose event or alarm).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogbookEntry {
    #[serde(rename = "FactoryTimestamp")]
    pub factory_timestamp: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "type")]
    pub entry_type: i32,
    #[serde(rename = "ValueInMgPerDl")]
    pub value_in_mg_per_dl: f64,
    #[serde(rename = "MeasurementColor")]
    pub measurement_color: i32,
    #[serde(rename = "GlucoseUnits")]
    pub glucose_units: i32,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "isHigh")]
    pub is_high: bool,
    #[serde(rename = "isLow")]
    pub is_low: bool,
    #[serde(rename = "TrendArrow")]
    pub trend_arrow: i32,
    #[serde(rename = "TrendMessage")]
    pub trend_message: Option<String>,
    #[serde(rename = "alarmType")]
    pub alarm_type: i32,
}

/// Response from GET /llu/connections/{patientId}/logbook (authenticated).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogbookResponse {
    pub status: i32,
    pub data: Vec<LogbookEntry>,
    pub ticket: AuthTicket,
}
