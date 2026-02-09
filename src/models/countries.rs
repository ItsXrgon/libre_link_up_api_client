use serde::{Deserialize, Serialize};

/// Regional endpoint configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AE {
    /// LSL API endpoint
    pub lsl_api: String,
    /// Socket Hub endpoint
    pub socket_hub: String,
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
