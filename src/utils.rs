use crate::models::{
    LibreCgmData,
    client::TrendType,
    common::{GlucoseItem, GlucoseMeasurement},
};
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

/// Trait for types that can be converted to LibreCgmData
pub trait GlucoseData {
    fn factory_timestamp(&self) -> &str;
    fn value(&self) -> f64;
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
    fn trend_arrow(&self) -> Option<i32>;
}

impl GlucoseData for GlucoseItem {
    fn factory_timestamp(&self) -> &str {
        &self.factory_timestamp
    }
    fn value(&self) -> f64 {
        self.value
    }
    fn is_high(&self) -> bool {
        self.is_high
    }
    fn is_low(&self) -> bool {
        self.is_low
    }
    fn trend_arrow(&self) -> Option<i32> {
        self.trend_arrow
    }
}

impl GlucoseData for GlucoseMeasurement {
    fn factory_timestamp(&self) -> &str {
        &self.factory_timestamp
    }
    fn value(&self) -> f64 {
        self.value
    }
    fn is_high(&self) -> bool {
        self.is_high
    }
    fn is_low(&self) -> bool {
        self.is_low
    }
    fn trend_arrow(&self) -> Option<i32> {
        Some(self.trend_arrow)
    }
}

/// Convert glucose data to LibreCgmData
pub fn map_glucose_data<T: GlucoseData>(item: &T) -> LibreCgmData {
    let date = format!("{} UTC", item.factory_timestamp())
        .parse()
        .unwrap_or_else(|_| Utc::now());

    LibreCgmData {
        value: item.value(),
        is_high: item.is_high(),
        is_low: item.is_low(),
        trend: get_trend(item.trend_arrow()),
        date,
    }
}
