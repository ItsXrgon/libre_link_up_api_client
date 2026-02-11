//! Data models for LibreLinkUp API responses and requests.
//!
//! Types are grouped by endpoint or domain: [client], [common], [connection], [connections], [countries],
//! [graph], [logbook], [login], [notifications], [region].

pub mod client;
pub mod common;
pub mod connection;
pub mod connections;
pub mod countries;
pub mod graph;
pub mod logbook;
pub mod login;
pub mod notifications;
pub mod region;

pub use client::{LibreCgmData, ReadRawResponse, ReadResponse, TrendType};
pub use common::{
    ActiveSensor, AlarmRules, AuthTicket, Connection, F, FixedLowAlarmValues, GlucoseItem,
    GlucoseMeasurement, H, L, Nd, PatientDevice, Sensor, Std,
};
pub use connection::{ConnectionData, ConnectionResponse};
pub use connections::ConnectionsResponse;
pub use countries::{
    AE, CountryConfigData, CountryConfigResponse, CountryEntry, CountryList, CountryResponse,
    RegionalMap,
};
pub use graph::{GraphData, GraphResponse};
pub use logbook::{LogbookEntry, LogbookResponse};
pub use login::{
    AccountResponse, Data as LoginData, LoginArgs, LoginRedirectResponse, LoginResponse,
    LoginResponseData, StepData, User, UserResponse,
};
pub use notifications::{
    NotificationSettingsAlarmRules, NotificationSettingsData, NotificationSettingsL,
    NotificationSettingsNd, NotificationSettingsPatientDevice, NotificationSettingsResponse,
};
pub use region::Region;
