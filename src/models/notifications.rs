//! Notification settings types for GET /llu/notifications/settings/{connectionId}.

use crate::models::common::{AuthTicket, F, FixedLowAlarmValues, H, Std};
use serde::{Deserialize, Serialize};

/// Low glucose alarm configuration for notifications settings (includes `on` field).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettingsL {
    /// Whether this alarm rule is enabled
    pub on: bool,
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

/// No data alarm configuration for notifications settings (includes "on" field).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettingsNd {
    /// Whether this alarm rule is enabled
    pub on: bool,
    /// Interval
    pub i: i32,
    /// Repeat
    pub r: i32,
    /// Limit
    pub l: i32,
}

/// Alarm rules configuration for notifications settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettingsAlarmRules {
    /// Critical alarm enabled
    pub c: bool,
    /// High glucose alarm configuration
    pub h: H,
    /// Falling glucose alarm configuration
    pub f: F,
    /// Low glucose alarm configuration
    pub l: NotificationSettingsL,
    /// No data alarm configuration
    pub nd: NotificationSettingsNd,
    /// Period
    pub p: i32,
    /// Repeat
    pub r: i32,
    /// Standard alarm configuration
    pub std: Std,
}

/// Patient device configuration for notifications settings (includes additional fields).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettingsPatientDevice {
    /// Device identifier
    pub did: String,
    /// Device type identifier
    pub dtid: i32,
    /// Device version
    pub v: String,
    /// Low limit enabled
    pub l: bool,
    /// Low limit threshold
    pub ll: f64,
    /// High limit enabled
    pub h: bool,
    /// High limit threshold
    pub hl: f64,
    /// Unit of measure
    pub u: i32,
    /// Fixed low alarm values
    #[serde(rename = "fixedLowAlarmValues")]
    pub fixed_low_alarm_values: FixedLowAlarmValues,
    /// Whether alarms are enabled
    pub alarms: bool,
    /// Fixed low threshold
    #[serde(rename = "fixedLowThreshold")]
    pub fixed_low_threshold: f64,
}

/// Data payload for GET /llu/notifications/settings/{connectionId}.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettingsData {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "alarmRules")]
    pub alarm_rules: NotificationSettingsAlarmRules,
    pub std: Std,
    #[serde(rename = "patientDevice")]
    pub patient_device: NotificationSettingsPatientDevice,
}

/// Response from GET /llu/notifications/settings/{connectionId} (authenticated).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSettingsResponse {
    pub status: i32,
    pub data: NotificationSettingsData,
    pub ticket: AuthTicket,
}
