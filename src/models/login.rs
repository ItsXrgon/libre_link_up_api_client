use crate::models::common::AuthTicket;
use serde::{Deserialize, Serialize};

/// Login credentials
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginArgs {
    #[serde(rename = "email")]
    pub username: String,
    pub password: String,
}

/// Login redirect response when regional redirect is needed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRedirectResponse {
    pub status: i32,
    pub data: LoginRedirectData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginRedirectData {
    pub redirect: bool,
    pub region: String,
}

/// Main login response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub status: i32,
    pub data: LoginResponseData,
}

/// Login response data - can be either complete user data or step data for MFA
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginResponseData {
    Complete(Box<Data>),
    Redirect(LoginRedirectData),
    Step(StepData),
    Locked(LockedData),
}

/// Locked account data for rate limiting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LockedData {
    pub code: i32,
    pub data: LockoutInfo,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LockoutInfo {
    pub failures: i32,
    pub interval: i32,
    pub lockout: i32,
}

/// Step data for additional authentication requirements (MFA, email verification, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepData {
    pub step: Step,
    pub user: StepUser,
    #[serde(rename = "authTicket")]
    pub auth_ticket: AuthTicket,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Step {
    #[serde(rename = "type")]
    pub step_type: String,
    #[serde(rename = "componentName")]
    pub component_name: String,
    pub props: StepProps,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepProps {
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepUser {
    pub id: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    pub country: String,
    #[serde(rename = "uiLanguage")]
    pub ui_language: String,
}

/// Complete login data with full user information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub user: User,
    pub messages: DataMessages,
    pub notifications: Notifications,
    #[serde(rename = "authTicket")]
    pub auth_ticket: AuthTicket,
    #[serde(default)]
    pub invitations: Option<Vec<String>>,
    #[serde(default, rename = "trustedDeviceToken")]
    pub trusted_device_token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DataMessages {
    pub unread: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Notifications {
    pub unresolved: i32,
}

/// User profile information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct User {
    pub id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: String,
    pub country: String,
    #[serde(rename = "uiLanguage")]
    pub ui_language: String,
    #[serde(rename = "communicationLanguage")]
    pub communication_language: String,
    #[serde(rename = "accountType")]
    pub account_type: String,
    pub uom: String,
    #[serde(rename = "dateFormat")]
    pub date_format: String,
    #[serde(rename = "timeFormat")]
    pub time_format: String,
    #[serde(default, rename = "emailDay")]
    pub email_day: Vec<i32>,
    #[serde(default)]
    pub system: System,
    #[serde(default)]
    pub details: Details,
    #[serde(default, rename = "twoFactor")]
    pub two_factor: Option<TwoFactor>,
    #[serde(default)]
    pub created: i64,
    #[serde(default, rename = "lastLogin")]
    pub last_login: i64,
    #[serde(default)]
    pub programs: Details,
    #[serde(default, rename = "dateOfBirth")]
    pub date_of_birth: i64,
    #[serde(default)]
    pub practices: Details,
    #[serde(default)]
    pub devices: Details,
    #[serde(default)]
    pub consents: Consents,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TwoFactor {
    #[serde(rename = "primaryMethod")]
    pub primary_method: String,
    #[serde(rename = "primaryValue")]
    pub primary_value: String,
    #[serde(rename = "secondaryMethod")]
    pub secondary_method: String,
    #[serde(rename = "secondaryValue")]
    pub secondary_value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Consents {
    #[serde(default)]
    pub llu: Llu,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Llu {
    #[serde(rename = "policyAccept")]
    pub policy_accept: i64,
    #[serde(rename = "touAccept")]
    pub tou_accept: i64,
}

/// Empty details object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Details {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct System {
    pub messages: SystemMessages,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SystemMessages {
    #[serde(rename = "firstUsePhoenix")]
    pub first_use_phoenix: Option<i64>,
    #[serde(rename = "firstUsePhoenixReportsDataMerged")]
    pub first_use_phoenix_reports_data_merged: Option<i64>,
    #[serde(rename = "lluGettingStartedBanner")]
    pub llu_getting_started_banner: Option<i64>,
    #[serde(rename = "lluNewFeatureModal")]
    pub llu_new_feature_modal: Option<i64>,
    #[serde(rename = "lluOnboarding")]
    pub llu_onboarding: Option<i64>,
    #[serde(rename = "lvWebPostRelease")]
    pub lv_web_post_release: Option<String>,
    #[serde(rename = "appReviewBanner")]
    pub app_review_banner: Option<i64>,
    #[serde(rename = "streamingTourMandatory")]
    pub streaming_tour_mandatory: Option<i64>,
}
