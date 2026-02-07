use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibreLinkUpError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error(
        "Bad credentials. Please ensure that you have entered the credentials of your LibreLinkUp account (and not of your LibreLink account)."
    )]
    BadCredentials,

    #[error(
        "Account temporarily locked due to multiple failed login attempts. Please wait {0} seconds and try again."
    )]
    AccountLocked(i32),

    #[error(
        "Additional action required for your account: {0}. Please login via app and perform required steps and try again."
    )]
    AdditionalActionRequired(String),

    #[error("Unable to find region '{0}'. Available regions: {1}")]
    RegionNotFound(String, String),

    #[error("Unable to identify connection by given name '{0}'")]
    ConnectionNotFound(String),

    #[error("Unable to identify connection by given function")]
    ConnectionFunctionFailed,

    #[error("Your account does not follow any patients. Please start following and try again.")]
    NoConnections,

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid API response: {0}")]
    InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, LibreLinkUpError>;
