//! Country/region config types for GET /llu/config/country.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Regional endpoint configuration (LSL API + Socket Hub).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AE {
    /// LSL API endpoint
    pub lsl_api: String,
    /// Socket Hub endpoint
    pub socket_hub: String,
}

/// Single country entry in the country list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CountryEntry {
    pub display_member: String,
    pub value_member: String,
}

/// Country list wrapper from config
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CountryList {
    pub countries: Vec<CountryEntry>,
}

/// Data payload from GET /llu/config/country
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryConfigData {
    #[serde(default)]
    pub capture_analytics: Option<String>,
    #[serde(default)]
    pub country_list: Option<CountryList>,
    #[serde(default)]
    pub lsl_service_url: Option<String>,
    #[serde(default)]
    pub libre_link_resource_key: Option<String>,
    #[serde(default)]
    pub min_version: Option<String>,
    #[serde(default)]
    pub partner_application_keys: Option<Vec<String>>,
    #[serde(default)]
    pub show_alert: Option<bool>,
    #[serde(default)]
    pub show_android_badges: Option<String>,
    #[serde(default)]
    pub supported_languages: Option<Vec<String>>,
    #[serde(default)]
    pub regional_map: Option<HashMap<String, AE>>,
    #[serde(default)]
    pub alarms_enabled: Option<Vec<String>>,
    #[serde(default)]
    pub heartbeat_milliseconds: Option<u64>,
    #[serde(default)]
    pub llu_app_android: Option<String>,
    #[serde(default)]
    pub llu_app_ios: Option<String>,
    #[serde(default)]
    pub llu_sam: Option<String>,
    #[serde(default)]
    pub llu_support: Option<String>,
    #[serde(default)]
    pub llu_support_main: Option<String>,
    #[serde(default)]
    pub lsl_api: Option<String>,
    #[serde(default)]
    pub lv: Option<String>,
    #[serde(default)]
    pub minority_age: Option<u32>,
    #[serde(default)]
    pub name_order: Option<Vec<String>>,
    #[serde(default)]
    pub notification_service: Option<String>,
    #[serde(default)]
    pub safety_banner_interval: Option<u32>,
}

/// Response from GET /llu/config/country (unauthenticated). Use [`LibreLinkUpClient::get_country_config`](crate::LibreLinkUpClient::get_country_config) to fetch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryConfigResponse {
    pub status: i32,
    pub data: CountryConfigData,
}

/// Map of all regional endpoints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionalMap {
    /// United States endpoint
    pub us: AE,
    /// Europe endpoint
    pub eu: AE,
    /// France endpoint
    pub fr: AE,
    /// Japan endpoint
    pub jp: AE,
    /// Germany endpoint
    pub de: AE,
    /// Asia-Pacific endpoint
    pub ap: AE,
    /// Australia endpoint
    pub au: AE,
    /// United Arab Emirates endpoint
    pub ae: AE,
    /// Europe 2 endpoint
    pub eu2: AE,
    /// Canada endpoint
    pub ca: AE,
    /// Latin America endpoint
    pub la: AE,
    /// Russia endpoint
    pub ru: AE,
    /// China endpoint
    pub cn: AE,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub regional_map: RegionalMap,
}

/// Response from the country/region configuration endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryResponse {
    /// HTTP status code
    pub status: i32,
    /// Data containing regional map
    pub data: Data,
}
