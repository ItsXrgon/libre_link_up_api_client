//! Common data structures shared across multiple API endpoints

use serde::{Deserialize, Serialize};

/// Authentication ticket with token and expiration information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthTicket {
    /// JWT authentication token
    pub token: String,
    /// Unix timestamp when the token expires
    #[serde(default)]
    pub expires: i64,
    /// Duration in seconds for which the token is valid
    #[serde(default)]
    pub duration: i64,
}

/// Sensor device information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sensor {
    /// Device identifier
    #[serde(rename = "deviceId")]
    pub device_id: String,
    /// Serial number
    pub sn: String,
    /// Sensor age (days)
    pub a: i32,
    /// Sensor warmup period (minutes)
    pub w: i32,
    /// Sensor type
    pub pt: i32,
}

/// Fixed low alarm threshold values in both units
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixedLowAlarmValues {
    /// Threshold in mg/dL
    pub mgdl: f64,
    /// Threshold in mmol/L
    pub mmoll: f64,
}

/// Patient device configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatientDevice {
    /// Device identifier
    pub did: String,
    /// Device type identifier
    pub dtid: i32,
    /// Device version
    pub v: String,
    /// Low limit threshold
    pub ll: f64,
    /// High limit threshold
    pub hl: f64,
    /// Unit of measure
    pub u: i32,
    /// Fixed low alarm values
    #[serde(rename = "fixedLowAlarmValues")]
    pub fixed_low_alarm_values: FixedLowAlarmValues,
    /// Whether alarms are enabled
    pub alarms: bool,
}

/// Alarm rule configuration for falling glucose (F)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct F {
    /// Threshold high
    pub th: f64,
    /// Threshold high in mmol/L
    pub thmm: f64,
    /// Duration
    pub d: i32,
    /// Threshold low
    pub tl: f64,
    /// Threshold low in mmol/L
    pub tlmm: f64,
}

/// Alarm rule configuration for low glucose (L)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct L {
    /// Threshold high
    pub th: f64,
    /// Threshold high in mmol/L
    pub thmm: f64,
    /// Duration
    pub d: i32,
    /// Threshold low
    pub tl: f64,
    /// Threshold low in mmol/L
    pub tlmm: f64,
}

/// Alarm rule configuration for high glucose (H)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct H {
    /// Whether this alarm rule is enabled
    pub on: bool,
    /// Threshold high
    pub th: f64,
    /// Threshold high in mmol/L
    pub thmm: f64,
    /// Duration
    pub d: i32,
    /// Frequency
    pub f: f64,
}

/// Alarm rule configuration for no data (Nd)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nd {
    /// Interval
    pub i: i32,
    /// Repeat
    pub r: i32,
    /// Limit
    pub l: i32,
}

/// Standard alarm rule configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Std {}

/// Alarm rules configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlarmRules {
    /// Critical alarm enabled
    pub c: Option<bool>,
    /// High glucose alarm configuration
    pub h: H,
    /// Falling glucose alarm configuration
    pub f: F,
    /// Low glucose alarm configuration
    pub l: L,
    /// No data alarm configuration
    pub nd: Nd,
    /// Period
    pub p: i32,
    /// Repeat
    pub r: i32,
    /// Standard alarm configuration
    pub std: Std,
}

/// Glucose measurement data item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlucoseItem {
    /// Factory timestamp of the reading
    #[serde(rename = "FactoryTimestamp")]
    pub factory_timestamp: String,
    /// Timestamp of the reading
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    /// Type of glucose reading
    #[serde(rename = "type")]
    pub item_type: i32,
    /// Value in mg/dL
    #[serde(rename = "ValueInMgPerDl")]
    pub value_in_mg_per_dl: f64,
    /// Trend arrow direction (0-6, None if not available)
    #[serde(rename = "TrendArrow")]
    pub trend_arrow: Option<i32>,
    /// Trend message (if available)
    #[serde(rename = "TrendMessage")]
    pub trend_message: Option<serde_json::Value>,
    /// Measurement color indicator
    #[serde(rename = "MeasurementColor")]
    pub measurement_color: i32,
    /// Glucose units (0 = mg/dL, 1 = mmol/L)
    #[serde(rename = "GlucoseUnits")]
    pub glucose_units: i32,
    /// Glucose value in configured units
    #[serde(rename = "Value")]
    pub value: f64,
    /// Whether the value is above target high
    #[serde(rename = "isHigh")]
    pub is_high: bool,
    /// Whether the value is below target low
    #[serde(rename = "isLow")]
    pub is_low: bool,
}

/// Glucose measurement with required trend arrow
///
/// Extends `GlucoseItem` but requires `TrendArrow` to be present (not optional).
/// Used in connection endpoints where trend data is always available.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlucoseMeasurement {
    /// Factory timestamp of the reading
    #[serde(rename = "FactoryTimestamp")]
    pub factory_timestamp: String,
    /// Timestamp of the reading
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    /// Type of glucose reading
    #[serde(rename = "type")]
    pub item_type: i32,
    /// Value in mg/dL
    #[serde(rename = "ValueInMgPerDl")]
    pub value_in_mg_per_dl: f64,
    /// Trend arrow direction (0-6, always present)
    #[serde(rename = "TrendArrow")]
    pub trend_arrow: i32,
    /// Trend message (if available)
    #[serde(rename = "TrendMessage")]
    pub trend_message: Option<serde_json::Value>,
    /// Measurement color indicator
    #[serde(rename = "MeasurementColor")]
    pub measurement_color: i32,
    /// Glucose units (0 = mg/dL, 1 = mmol/L)
    #[serde(rename = "GlucoseUnits")]
    pub glucose_units: i32,
    /// Glucose value in configured units
    #[serde(rename = "Value")]
    pub value: f64,
    /// Whether the value is above target high
    #[serde(rename = "isHigh")]
    pub is_high: bool,
    /// Whether the value is below target low
    #[serde(rename = "isLow")]
    pub is_low: bool,
}

/// Active sensor with associated device
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveSensor {
    /// Sensor information
    pub sensor: Sensor,
    /// Patient device information
    pub device: PatientDevice,
}

/// Connection information for a patient
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    /// Connection identifier
    pub id: String,
    /// Patient identifier
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// Country code
    pub country: String,
    /// Connection status
    pub status: i32,
    /// Patient's first name
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Patient's last name
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Target low glucose value
    #[serde(rename = "targetLow")]
    pub target_low: f64,
    /// Target high glucose value
    #[serde(rename = "targetHigh")]
    pub target_high: f64,
    /// Unit of measure
    pub uom: i32,
    /// Sensor information
    pub sensor: Sensor,
    /// Alarm rules configuration
    #[serde(rename = "alarmRules")]
    pub alarm_rules: AlarmRules,
    /// Current glucose measurement (with required trend arrow)
    #[serde(rename = "glucoseMeasurement")]
    pub glucose_measurement: GlucoseMeasurement,
    /// Current glucose item (with optional trend arrow)
    #[serde(rename = "glucoseItem")]
    pub glucose_item: GlucoseItem,
    /// Glucose alarm information (if any)
    #[serde(rename = "glucoseAlarm")]
    pub glucose_alarm: Option<serde_json::Value>,
    /// Patient device information
    #[serde(rename = "patientDevice")]
    pub patient_device: PatientDevice,
    /// Creation timestamp
    pub created: i64,
}
