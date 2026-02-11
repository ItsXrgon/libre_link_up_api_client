//! HTTP client and configuration for the LibreLinkUp API.
//!
//! Entry points: [`LibreLinkUpClient`] (authenticated and unauthenticated requests).

use crate::{
    errors::{LibreLinkUpError, Result},
    models::{
        client::{LibreCgmData, ReadRawResponse, ReadResponse},
        common::Connection,
        connections::ConnectionsResponse,
        countries::CountryConfigResponse,
        graph::GraphResponse,
        logbook::LogbookResponse,
        login::{AccountResponse, LoginArgs, LoginResponse, LoginResponseData, UserResponse},
        notifications::NotificationSettingsResponse,
        region::Region,
    },
    utils::{TREND_MAP, map_glucose_data},
};
use reqwest::{Client, header};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

/// API Region configuration
const LOGIN_ENDPOINT: &str = "/llu/auth/login";
const CONNECTIONS_ENDPOINT: &str = "/llu/connections";
const COUNTRY_CONFIG_ENDPOINT: &str = "/llu/config/country";
const USER_ENDPOINT: &str = "/user";
const ACCOUNT_ENDPOINT: &str = "/account";
const NOTIFICATIONS_SETTINGS_ENDPOINT: &str = "/llu/notifications/settings";

/// Type alias for connection identifier function
type ConnectionFn = Arc<dyn Fn(&[Connection]) -> Option<String> + Send + Sync>;

/// Client configuration options
///
/// # Examples
///
/// ```
/// use libre_link_up_api_client::{ClientConfig, Region};
///
/// let config = ClientConfig {
///     username: "email@example.com".to_string(),
///     password: "password".to_string(),
///     api_version: None,  // Uses default "4.16.0"
///     region: Some(Region::US),
///     connection_identifier: None,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Username for LibreLinkUp account
    pub username: String,
    /// Password for LibreLinkUp account
    pub password: String,
    /// API version (defaults to "4.16.0")
    pub api_version: Option<String>,
    /// API region (defaults to Global which auto-redirects)
    pub region: Option<Region>,
    /// Optional connection identifier for multi-patient accounts
    pub connection_identifier: Option<ConnectionIdentifier>,
}

/// Connection identifier for multi-patient accounts
///
/// Choose a specific patient's data when following multiple people.
///
/// # Examples
///
/// ```
/// use libre_link_up_api_client::ConnectionIdentifier;
///
/// // By patient name
/// let by_name = ConnectionIdentifier::ByName("John Doe".to_string());
///
/// // By custom function
/// let by_fn = ConnectionIdentifier::ByFunction(
///     std::sync::Arc::new(|connections| {
///         connections.first().map(|c| c.patient_id.clone())
///     })
/// );
/// ```
#[derive(Clone)]
pub enum ConnectionIdentifier {
    /// Identify patient by first name, last name, or full name
    ByName(String),
    /// Identify patient using a custom function
    ByFunction(ConnectionFn),
}

impl std::fmt::Debug for ConnectionIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ByName(name) => write!(f, "ByName({})", name),
            Self::ByFunction(_) => write!(f, "ByFunction(<closure>)"),
        }
    }
}

/// Main LibreLinkUp API client
///
/// Handles authentication, token management, and API requests. The same client supports both
/// **authenticated** and **unauthenticated** calls:
///
/// - **Authenticated** (require login; token and account-id are sent): [`read`](Self::read),
///   [`read_raw`](Self::read_raw), [`get_user`](Self::get_user), [`get_account`](Self::get_account),
///   [`get_logbook`](Self::get_logbook), [`get_notification_settings`](Self::get_notification_settings).
///   These use automatic login and token refresh.
/// - **Unauthenticated** (no credentials sent): [`get_country_config`](Self::get_country_config).
///
/// # Examples
///
/// ```no_run
/// use libre_link_up_api_client::{LibreLinkUpClient, ClientConfig, Region};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Simple creation
/// let client = LibreLinkUpClient::simple(
///     "email@example.com".to_string(),
///     "password".to_string(),
///     None,
/// )?;
///
/// // With configuration
/// let config = ClientConfig {
///     username: "email@example.com".to_string(),
///     password: "password".to_string(),
///     api_version: None,
///     region: Some(Region::EU),
///     connection_identifier: None,
/// };
/// let client = LibreLinkUpClient::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct LibreLinkUpClient {
    config: ClientConfig,
    client: Client,
    base_url: Arc<RwLock<String>>,
    jwt_token: Arc<RwLock<Option<String>>>,
    account_id: Arc<RwLock<Option<String>>>,
    connection_id: Arc<RwLock<Option<String>>>,
}

impl LibreLinkUpClient {
    /// Create a new LibreLinkUp client with full configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Client configuration including credentials, region, and API version
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be built.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libre_link_up_api_client::{LibreLinkUpClient, ClientConfig, Region};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ClientConfig {
    ///     username: "email@example.com".to_string(),
    ///     password: "password".to_string(),
    ///     api_version: None,
    ///     region: Some(Region::EU),
    ///     connection_identifier: None,
    /// };
    ///
    /// let client = LibreLinkUpClient::new(config)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(config: ClientConfig) -> Result<Self> {
        // Basic validation to avoid confusing HTTP-level errors later
        if config.username.trim().is_empty() {
            return Err(LibreLinkUpError::AuthFailed(
                "username must not be empty".to_string(),
            ));
        }
        if config.password.is_empty() {
            return Err(LibreLinkUpError::AuthFailed(
                "password must not be empty".to_string(),
            ));
        }

        let version = config
            .api_version
            .clone()
            .unwrap_or_else(|| "4.16.0".to_string());

        let region = config.region.unwrap_or_default();
        let base_url_str = region.base_url().to_string();

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, "Mozilla/5.0 (iPhone; CPU OS 17_4.1 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/17.4.1 Mobile/10A5355d Safari/8536.25".parse().unwrap());
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        headers.insert("accept-encoding", "gzip".parse().unwrap());
        headers.insert("cache-control", "no-cache".parse().unwrap());
        headers.insert("connection", "Keep-Alive".parse().unwrap());
        headers.insert(
            header::CONTENT_TYPE,
            "application/json;charset=UTF-8".parse().unwrap(),
        );
        headers.insert("product", "llu.ios".parse().unwrap());
        headers.insert("version", version.parse().unwrap());
        headers.insert("accept-language", "en-US".parse().unwrap());

        let client: Client = Client::builder()
            .default_headers(headers)
            .gzip(true)
            .build()?;

        Ok(Self {
            config,
            client,
            base_url: Arc::new(RwLock::new(base_url_str)),
            jwt_token: Arc::new(RwLock::new(None)),
            account_id: Arc::new(RwLock::new(None)),
            connection_id: Arc::new(RwLock::new(None)),
        })
    }

    /// Create a simple client with just username and password
    ///
    /// Convenience constructor using default settings.
    ///
    /// # Arguments
    ///
    /// * `username` - LibreLinkUp account email
    /// * `password` - LibreLinkUp account password
    /// * `region` - Optional region string (e.g., "us", "eu"). Auto-detects if None.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be built.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "email@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn simple(username: String, password: String, region: Option<String>) -> Result<Self> {
        if username.trim().is_empty() {
            return Err(LibreLinkUpError::AuthFailed(
                "username must not be empty".to_string(),
            ));
        }
        if password.is_empty() {
            return Err(LibreLinkUpError::AuthFailed(
                "password must not be empty".to_string(),
            ));
        }

        let region_enum = region
            .as_deref()
            .and_then(|s| Region::from_str(s).ok())
            .or(Some(Region::default()));

        Self::new(ClientConfig {
            username,
            password,
            api_version: None,
            region: region_enum,
            connection_identifier: None,
        })
    }

    /// Login to the LibreLinkUp service
    async fn login(&self) -> Result<LoginResponse> {
        let base_url = self.base_url.read().await.clone();
        let url = format!("{}{}", base_url, LOGIN_ENDPOINT);

        let login_args = LoginArgs {
            username: self.config.username.clone(),
            password: self.config.password.clone(),
        };

        let response = self.client.post(&url).json(&login_args).send().await?;

        // Check if response is successful
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response".to_string());
            return Err(LibreLinkUpError::InvalidResponse(format!(
                "Login failed - HTTP {}: {}",
                status, text
            )));
        }

        // Try to parse JSON with better error handling
        let text = response.text().await?;

        let login_response: LoginResponse = serde_json::from_str(&text).map_err(|e| {
            LibreLinkUpError::InvalidResponse(format!("Failed to parse JSON: {}", e))
        })?;

        // Check for account lockout
        if let LoginResponseData::Locked(locked_data) = &login_response.data {
            return Err(LibreLinkUpError::AccountLocked(locked_data.data.lockout));
        }

        // Check for bad credentials
        if login_response.status == 2 {
            return Err(LibreLinkUpError::BadCredentials);
        }

        // Check for additional action required (MFA, etc.)
        if login_response.status == 4 {
            let component_name = match &login_response.data {
                LoginResponseData::Step(step_data) => step_data.step.component_name.clone(),
                _ => "unknown".to_string(),
            };
            return Err(LibreLinkUpError::AdditionalActionRequired(component_name));
        }

        // Handle regional redirect
        if let LoginResponseData::Redirect(redirect_data) = &login_response.data
            && redirect_data.redirect
        {
            return self.handle_redirect(redirect_data.region.clone()).await;
        }

        // Extract token and account ID
        if let LoginResponseData::Complete(data) = &login_response.data {
            *self.jwt_token.write().await = Some(data.auth_ticket.token.clone());
            *self.account_id.write().await = Some(data.user.id.clone());
        }

        Ok(login_response)
    }

    /// Handle regional redirect during login
    async fn handle_redirect(&self, region: String) -> Result<LoginResponse> {
        // Parse region string (FromStr never fails, defaults to Global)
        let region_enum = Region::from_str(&region).unwrap();
        let region_url = region_enum.base_url().to_string();
        *self.base_url.write().await = region_url;

        // Retry login with new region (using Box::pin for recursion)
        Box::pin(self.login()).await
    }

    /// Make an authenticated request with automatic re-authentication
    async fn authenticated_request<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        // Ensure we're logged in
        if self.jwt_token.read().await.is_none() {
            self.login().await?;
        }

        match self.try_request(path).await {
            Ok(response) => Ok(response),
            Err(_) => {
                // Re-authenticate and retry
                self.login().await?;
                self.try_request(path).await
            }
        }
    }

    /// Try to make a request with current authentication
    async fn try_request<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let base_url = self.base_url.read().await.clone();
        let url = format!("{}{}", base_url, path);

        let jwt_token = self.jwt_token.read().await.clone();
        let account_id = self.account_id.read().await.clone();

        let mut request = self.client.get(&url);

        if let Some(token) = jwt_token {
            request = request.header(header::AUTHORIZATION, format!("Bearer {}", token));
        }

        // Add SHA-256 hashed account-id header if available
        if let Some(id) = account_id {
            let mut hasher = Sha256::new();
            hasher.update(id.as_bytes());
            let hashed_id = format!("{:x}", hasher.finalize());
            request = request.header("account-id", hashed_id);
        }

        let response = request.send().await?;

        // Check if response is successful
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response".to_string());
            return Err(LibreLinkUpError::InvalidResponse(format!(
                "request to '{}' failed - HTTP {}: {}",
                path, status, text
            )));
        }

        // Try to parse JSON, with better error message on failure
        let text = response.text().await?;
        serde_json::from_str(&text).map_err(|e| {
            LibreLinkUpError::InvalidResponse(format!("failed to parse JSON for '{}': {}", path, e))
        })
    }

    /// Make an unauthenticated GET request (no Bearer token or account-id).
    /// Use for endpoints that do not require login (e.g. country config).
    async fn unauthenticated_get<T: DeserializeOwned>(
        &self,
        url: &str,
        path_label: &str,
    ) -> Result<T> {
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response".to_string());
            return Err(LibreLinkUpError::InvalidResponse(format!(
                "request to '{}' failed - HTTP {}: {}",
                path_label, status, body
            )));
        }
        let body: String = response.text().await?;
        serde_json::from_str(&body).map_err(|e| {
            LibreLinkUpError::InvalidResponse(format!(
                "failed to parse JSON for '{}': {}",
                path_label, e
            ))
        })
    }

    /// Get list of connections
    async fn get_connections(&self) -> Result<ConnectionsResponse> {
        self.authenticated_request(CONNECTIONS_ENDPOINT).await
    }

    /// Get current user profile (authenticated).
    ///
    /// Returns user info, messages, notifications, and auth ticket.
    ///
    /// # Errors
    ///
    /// Returns [`LibreLinkUpError::Http`] or [`LibreLinkUpError::InvalidResponse`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "user@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    /// let user = client.get_user().await?;
    /// println!("{} {}", user.data.user.first_name, user.data.user.last_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user(&self) -> Result<UserResponse> {
        self.authenticated_request(USER_ENDPOINT).await
    }

    /// Get account info (authenticated).
    ///
    /// Returns user profile and current auth ticket.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "user@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    /// let account = client.get_account().await?;
    /// println!("{} {}", account.data.user.first_name, account.data.user.last_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_account(&self) -> Result<AccountResponse> {
        self.authenticated_request(ACCOUNT_ENDPOINT).await
    }

    /// Get logbook (glucose events/alarms) for a patient (authenticated).
    ///
    /// # Arguments
    ///
    /// * `patient_id` - Patient/connection ID (same as used for [`read`](Self::read) / [`read_raw`](Self::read_raw)).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "user@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    /// let logbook = client.get_logbook("patient-id").await?;
    /// for entry in &logbook.data {
    ///     println!("{} {}", entry.timestamp, entry.value);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_logbook(&self, patient_id: &str) -> Result<LogbookResponse> {
        let path = format!("{}/{}/logbook", CONNECTIONS_ENDPOINT, patient_id);
        self.authenticated_request(&path).await
    }

    /// Get notification settings for a connection (authenticated).
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Connection/patient ID (same as used for [`read`](Self::read) / [`read_raw`](Self::read_raw)).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "user@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    /// let settings = client.get_notification_settings("connection-id").await?;
    /// println!("Alarms enabled: {}", settings.data.alarm_rules.c);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_notification_settings(
        &self,
        connection_id: &str,
    ) -> Result<NotificationSettingsResponse> {
        let path = format!("{}/{}", NOTIFICATIONS_SETTINGS_ENDPOINT, connection_id);
        self.authenticated_request(&path).await
    }

    /// Fetch country/region config (unauthenticated).
    ///
    /// Uses the global API endpoint. Does not require login.
    ///
    /// # Arguments
    ///
    /// * `country` - Country code (e.g. `"us"`, `"eu"`).
    /// * `version` - API version (e.g. `"4.16.0"`). If `None`, uses the client's configured version.
    ///
    /// # Errors
    ///
    /// Returns [`LibreLinkUpError::Http`] or [`LibreLinkUpError::InvalidResponse`] on failure.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "user@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    /// let config = client.get_country_config("us", None).await?;
    /// println!("Min version: {:?}", config.data.min_version);
    /// println!("Regional map: {:?}", config.data.regional_map);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_country_config(
        &self,
        country: &str,
        version: Option<&str>,
    ) -> Result<CountryConfigResponse> {
        let version =
            version.unwrap_or_else(|| self.config.api_version.as_deref().unwrap_or("4.16.0"));
        let url = format!(
            "{}{}?country={}&version={}",
            Region::Global.base_url(),
            COUNTRY_CONFIG_ENDPOINT,
            country,
            version,
        );
        self.unauthenticated_get(&url, COUNTRY_CONFIG_ENDPOINT)
            .await
    }

    /// Get connection ID by identifier
    fn get_connection_id(&self, connections: &[Connection]) -> Result<String> {
        match &self.config.connection_identifier {
            Some(ConnectionIdentifier::ByName(name)) => {
                let connection = connections
                    .iter()
                    .find(|c| {
                        format!("{} {}", c.first_name, c.last_name).to_lowercase()
                            == name.to_lowercase()
                    })
                    .ok_or_else(|| LibreLinkUpError::ConnectionNotFound(name.clone()))?;

                Ok(connection.patient_id.clone())
            }
            Some(ConnectionIdentifier::ByFunction(func)) => {
                func(connections).ok_or(LibreLinkUpError::ConnectionFunctionFailed)
            }
            None => {
                // Default to first connection
                connections
                    .first()
                    .map(|c| c.patient_id.clone())
                    .ok_or(LibreLinkUpError::NoConnections)
            }
        }
    }

    /// Read raw glucose data from the API
    ///
    /// Returns unparsed API responses with all available data including
    /// connection info, active sensors, and glucose measurements.
    ///
    /// # Errors
    ///
    /// - [`LibreLinkUpError::NoConnections`] if no patients are being followed
    /// - [`LibreLinkUpError::AuthFailed`] if authentication fails
    /// - [`LibreLinkUpError::Http`] for network errors
    /// - [`LibreLinkUpError::InvalidResponse`] if API response is malformed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "email@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    ///
    /// let raw = client.read_raw().await?;
    /// println!("Connection: {:?}", raw.connection);
    /// println!("Active sensors: {}", raw.active_sensors.len());
    /// println!("Graph data points: {}", raw.graph_data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_raw(&self) -> Result<ReadRawResponse> {
        let connection_id = if let Some(id) = self.connection_id.read().await.clone() {
            id
        } else {
            let connections = self.get_connections().await?;

            if connections.data.is_empty() {
                return Err(LibreLinkUpError::NoConnections);
            }

            let id = self.get_connection_id(&connections.data)?;
            *self.connection_id.write().await = Some(id.clone());
            id
        };

        let path = format!("{}/{}/graph", CONNECTIONS_ENDPOINT, connection_id);
        let graph_response: GraphResponse = self.authenticated_request(&path).await?;

        Ok(ReadRawResponse {
            connection: graph_response.data.connection,
            active_sensors: graph_response.data.active_sensors,
            graph_data: graph_response.data.graph_data,
        })
    }

    /// Read current and historical glucose data
    ///
    /// Returns processed glucose data with current reading and historical measurements.
    /// Automatically handles authentication and connection management.
    ///
    /// # Errors
    ///
    /// - [`LibreLinkUpError::NoConnections`] if no patients are being followed
    /// - [`LibreLinkUpError::AuthFailed`] if authentication fails
    /// - [`LibreLinkUpError::Http`] for network errors
    /// - [`LibreLinkUpError::InvalidResponse`] if API response is malformed
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "email@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    ///
    /// let data = client.read().await?;
    /// println!("Current glucose: {:.1} mg/dL", data.current.value);
    /// println!("Trend: {:?}", data.current.trend);
    /// println!("Historical readings: {}", data.history.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(&self) -> Result<ReadResponse> {
        let raw = self.read_raw().await?;

        Ok(ReadResponse {
            current: map_glucose_data(&raw.connection.glucose_measurement),
            history: raw.graph_data.iter().map(map_glucose_data).collect(),
        })
    }

    /// Read averaged glucose data over time
    ///
    /// Polls the API at regular intervals and calculates averages when the specified
    /// number of readings have been collected. The callback is invoked with the
    /// current reading, recent readings used for averaging, and full history.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of readings to collect before averaging
    /// * `callback` - Function called with (current, averaged_history, full_history)
    /// * `interval_ms` - Polling interval in milliseconds
    ///
    /// # Returns
    ///
    /// Returns a `JoinHandle` for the background polling task. Call `.abort()` on it to stop.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be cloned for background operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libre_link_up_api_client::LibreLinkUpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = LibreLinkUpClient::simple(
    ///     "email@example.com".to_string(),
    ///     "password".to_string(),
    ///     None,
    /// )?;
    ///
    /// let handle = client.read_averaged(
    ///     10,  // Average 10 readings
    ///     |current, averaged, history| {
    ///         println!("Current: {:.1} mg/dL", current.value);
    ///         let avg = averaged.iter().map(|d| d.value).sum::<f64>() / averaged.len() as f64;
    ///         println!("Average: {:.1} mg/dL", avg);
    ///     },
    ///     60000,  // Poll every 60 seconds
    /// ).await?;
    ///
    /// // Later: handle.abort() to stop polling
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_averaged<F>(
        &self,
        amount: usize,
        mut callback: F,
        interval_ms: u64,
    ) -> Result<tokio::task::JoinHandle<()>>
    where
        F: FnMut(LibreCgmData, Vec<LibreCgmData>, Vec<LibreCgmData>) + Send + 'static,
    {
        let client = Self::new(self.config.clone())?;

        let handle = tokio::spawn(async move {
            let mut memory: Vec<LibreCgmData> = Vec::new();
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_millis(interval_ms));

            loop {
                interval.tick().await;

                if let Ok(read_response) = client.read().await {
                    let current = read_response.current;
                    let history = read_response.history;

                    // Check if we already have this reading
                    if !memory.iter().any(|m| m.date == current.date) {
                        memory.push(current.clone());
                    }

                    if memory.len() >= amount {
                        // Calculate average
                        let avg_value =
                            memory.iter().map(|m| m.value).sum::<f64>() / memory.len() as f64;

                        let trend_indices: Vec<usize> = memory
                            .iter()
                            .filter_map(|m| TREND_MAP.iter().position(|&t| t == m.trend))
                            .collect();

                        let avg_trend_idx = if !trend_indices.is_empty() {
                            (trend_indices.iter().sum::<usize>() as f64
                                / trend_indices.len() as f64)
                                .round() as usize
                        } else {
                            3 // Default to Flat
                        };

                        let avg_trend = TREND_MAP
                            .get(avg_trend_idx)
                            .copied()
                            .unwrap_or(TREND_MAP[3]);

                        let averaged = LibreCgmData {
                            value: avg_value.round(),
                            is_high: current.is_high,
                            is_low: current.is_low,
                            trend: avg_trend,
                            date: current.date,
                        };

                        // Move the collected readings into the callback without cloning
                        let collected = std::mem::take(&mut memory);
                        callback(averaged, collected, history);
                    }
                }
            }
        });

        Ok(handle)
    }
}
