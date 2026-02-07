use crate::{
    errors::{LibreLinkUpError, Result},
    models::{
        client::LibreCgmData,
        connection::{ActiveSensor, Connection, GlucoseItem},
        connections::{ConnectionsResponse, Datum},
        graph::GraphData,
        login::{LoginArgs, LoginResponse, LoginResponseData},
        region::Region,
    },
    utils::{TREND_MAP, map_glucose_data},
};
use reqwest::{Client, header};
use serde::de::DeserializeOwned;
use std::{str::FromStr, sync::Arc};
use tokio::sync::RwLock;

/// API Region configuration
const LOGIN_ENDPOINT: &str = "/llu/auth/login";
const CONNECTIONS_ENDPOINT: &str = "/llu/connections";

/// Type alias for connection identifier function
type ConnectionFn = Arc<dyn Fn(&[Datum]) -> Option<String> + Send + Sync>;

/// Client configuration options
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub username: String,
    pub password: String,
    /// API version (defaults to "4.12.0")
    pub api_version: Option<String>,
    /// API region (defaults to Global)
    pub region: Option<Region>,
    pub connection_identifier: Option<ConnectionIdentifier>,
}

/// Connection identifier - either by name or by custom function
#[derive(Clone)]
pub enum ConnectionIdentifier {
    ByName(String),
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

/// Response from the read() method
#[derive(Debug, Clone)]
pub struct ReadResponse {
    pub current: LibreCgmData,
    pub history: Vec<LibreCgmData>,
}

/// Response from the read_raw() method
#[derive(Debug, Clone)]
pub struct ReadRawResponse {
    pub connection: Connection,
    pub active_sensors: Vec<ActiveSensor>,
    pub graph_data: Vec<GlucoseItem>,
}

/// Main LibreLinkUp API client
pub struct LibreLinkUpClient {
    config: ClientConfig,
    client: Client,
    base_url: Arc<RwLock<String>>,
    jwt_token: Arc<RwLock<Option<String>>>,
    account_id: Arc<RwLock<Option<String>>>,
    connection_id: Arc<RwLock<Option<String>>>,
}

impl LibreLinkUpClient {
    /// Create a new LibreLinkUp client with configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        let version = config
            .api_version
            .clone()
            .unwrap_or_else(|| "4.12.0".to_string());

        let region = config.region.unwrap_or_default();
        let base_url_str = region.base_url().to_string();

        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, "LibreLinkUp".parse().unwrap());
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        headers.insert("accept-encoding", "gzip".parse().unwrap());
        headers.insert("cache-control", "no-cache".parse().unwrap());
        headers.insert("connection", "Keep-Alive".parse().unwrap());
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("product", "llu.android".parse().unwrap());
        headers.insert("version", version.parse().unwrap());

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
    pub fn simple(username: String, password: String, region: Option<String>) -> Result<Self> {
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
    pub async fn login(&self) -> Result<LoginResponse> {
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
        if let LoginResponseData::Redirect(redirect_data) = &login_response.data {
            if redirect_data.redirect {
                return self.handle_redirect(redirect_data.region.clone()).await;
            }
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

        let mut request = self.client.get(&url);

        if let Some(token) = jwt_token {
            request = request.header(header::AUTHORIZATION, format!("Bearer {}", token));
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
                "HTTP {}: {}",
                status, text
            )));
        }

        // Try to parse JSON, with better error message on failure
        let text = response.text().await?;
        serde_json::from_str(&text)
            .map_err(|e| LibreLinkUpError::InvalidResponse(format!("Failed to parse JSON: {}", e)))
    }

    /// Get list of connections
    async fn get_connections(&self) -> Result<ConnectionsResponse> {
        self.authenticated_request(CONNECTIONS_ENDPOINT).await
    }

    /// Get connection ID by identifier
    fn get_connection_id(&self, connections: &[Datum]) -> Result<String> {
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
        let graph_data: GraphData = self.authenticated_request(&path).await?;

        // Note: GraphData uses its own type definitions which are identical to connection types
        // We need to convert through JSON to avoid type mismatch
        let connection: Connection =
            serde_json::from_value(serde_json::to_value(&graph_data.data.connection)?)?;
        let active_sensors: Vec<ActiveSensor> =
            serde_json::from_value(serde_json::to_value(&graph_data.data.active_sensors)?)?;
        let graph_data_items: Vec<GlucoseItem> =
            serde_json::from_value(serde_json::to_value(&graph_data.data.graph_data)?)?;

        Ok(ReadRawResponse {
            connection,
            active_sensors,
            graph_data: graph_data_items,
        })
    }

    /// Read current and historical glucose data
    pub async fn read(&self) -> Result<ReadResponse> {
        let raw = self.read_raw().await?;

        Ok(ReadResponse {
            current: map_glucose_data(&raw.connection.glucose_measurement),
            history: raw.graph_data.iter().map(map_glucose_data).collect(),
        })
    }

    /// Read averaged glucose data over time
    ///
    /// This method polls the API at regular intervals and calculates averages
    /// when the specified amount of readings have been collected.
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

                        let mem_clone = memory.clone();
                        memory.clear();

                        callback(averaged, mem_clone, history);
                    }
                }
            }
        });

        Ok(handle)
    }
}
