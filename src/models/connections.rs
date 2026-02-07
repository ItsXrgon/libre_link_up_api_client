use serde::{Deserialize, Serialize};

/// Response from the connections list endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    pub status: i32,
    pub data: Vec<Datum>,
    pub ticket: Ticket,
}

/// Individual connection (patient) in the list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datum {
    pub id: String,
    #[serde(rename = "patientId")]
    pub patient_id: String,
    pub country: String,
    pub status: i32,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "targetLow")]
    pub target_low: f64,
    #[serde(rename = "targetHigh")]
    pub target_high: f64,
    pub uom: i32,
    pub sensor: Sensor,
    #[serde(rename = "alarmRules")]
    pub alarm_rules: AlarmRules,
    #[serde(rename = "glucoseMeasurement")]
    pub glucose_measurement: Glucose,
    #[serde(rename = "glucoseItem")]
    pub glucose_item: Glucose,
    #[serde(rename = "glucoseAlarm")]
    pub glucose_alarm: Option<serde_json::Value>,
    #[serde(rename = "patientDevice")]
    pub patient_device: PatientDevice,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmRules {
    pub c: bool,
    pub h: H,
    pub f: F,
    pub l: F,
    pub nd: Nd,
    pub p: i32,
    pub r: i32,
    pub std: Std,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct F {
    pub th: f64,
    pub thmm: f64,
    pub d: i32,
    pub tl: f64,
    pub tlmm: f64,
    pub on: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct H {
    pub on: bool,
    pub th: f64,
    pub thmm: f64,
    pub d: i32,
    pub f: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nd {
    pub i: i32,
    pub r: i32,
    pub l: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Std {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glucose {
    #[serde(rename = "FactoryTimestamp")]
    pub factory_timestamp: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "type")]
    pub glucose_type: i32,
    #[serde(rename = "ValueInMgPerDl")]
    pub value_in_mg_per_dl: f64,
    #[serde(rename = "TrendArrow")]
    pub trend_arrow: i32,
    #[serde(rename = "TrendMessage")]
    pub trend_message: Option<serde_json::Value>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientDevice {
    pub did: String,
    pub dtid: i32,
    pub v: String,
    pub ll: f64,
    pub hl: f64,
    pub u: i32,
    #[serde(rename = "fixedLowAlarmValues")]
    pub fixed_low_alarm_values: FixedLowAlarmValues,
    pub alarms: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedLowAlarmValues {
    pub mgdl: f64,
    pub mmoll: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub sn: String,
    pub a: i32,
    pub w: i32,
    pub pt: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub token: String,
    pub expires: i64,
    pub duration: i64,
}
