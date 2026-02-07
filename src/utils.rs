use crate::models::{LibreCgmData, client::TrendType, connection::GlucoseItem};
use chrono::Utc;

/// Trend map array matching API values to trend types
pub const TREND_MAP: [TrendType; 7] = [
    TrendType::NotComputable,
    TrendType::SingleDown,
    TrendType::FortyFiveDown,
    TrendType::Flat,
    TrendType::FortyFiveUp,
    TrendType::SingleUp,
    TrendType::NotComputable,
];

/// Get trend from arrow value
pub fn get_trend(trend_arrow: Option<i32>) -> TrendType {
    trend_arrow
        .and_then(|arrow| TREND_MAP.get(arrow as usize).copied())
        .unwrap_or(TrendType::Flat)
}

/// Convert raw GlucoseItem to LibreCgmData
pub fn map_glucose_data(item: &GlucoseItem) -> LibreCgmData {
    let date_str = format!("{} UTC", item.factory_timestamp);
    let date = date_str.parse().unwrap_or_else(|_| Utc::now());

    LibreCgmData {
        value: item.value,
        is_high: item.is_high,
        is_low: item.is_low,
        trend: get_trend(item.trend_arrow),
        date,
    }
}

