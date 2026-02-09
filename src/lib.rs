//! # LibreLinkUp API Client
//!
//! Unofficial Rust client for the LibreLinkUp API - fetch CGM (Continuous Glucose Monitor)
//! data from FreeStyle Libre 2/3 devices through Abbott's sharing service.
//!
//! ## Features
//!
//! - Automatic authentication and token management
//! - Regional endpoint support (US, EU, JP, DE, FR, AP, AU, AE)
//! - Current and historical glucose readings
//! - Raw API access for advanced use cases
//! - Type-safe API with comprehensive error handling
//! - Async/await support with Tokio
//!
//! ## Quick Start
//!
//! ```no_run
//! use libre_link_up_api_client::LibreLinkUpClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your credentials
//!     let client = LibreLinkUpClient::simple(
//!         "your_email@example.com".to_string(),
//!         "your_password".to_string(),
//!         None,
//!     )?;
//!
//!     // Read current glucose data
//!     let data = client.read().await?;
//!
//!     println!("Current glucose: {:.1} mg/dL", data.current.value);
//!     println!("Trend: {:?}", data.current.trend);
//!     println!("Historical readings: {}", data.history.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage
//!
//! ### Regional Endpoints
//!
//! ```no_run
//! use libre_link_up_api_client::{LibreLinkUpClient, ClientConfig, Region};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ClientConfig {
//!     username: "email@example.com".to_string(),
//!     password: "password".to_string(),
//!     api_version: None,
//!     region: Some(Region::EU),
//!     connection_identifier: None,
//! };
//!
//! let client = LibreLinkUpClient::new(config)?;
//! let data = client.read().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Raw API Access
//!
//! ```no_run
//! use libre_link_up_api_client::LibreLinkUpClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = LibreLinkUpClient::simple(
//!     "email@example.com".to_string(),
//!     "password".to_string(),
//!     None,
//! )?;
//!
//! // Get raw API responses
//! let raw = client.read_raw().await?;
//! println!("Connection: {:?}", raw.connection);
//! println!("Active sensors: {:?}", raw.active_sensors);
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! All API operations return [`Result<T, LibreLinkUpError>`](errors::LibreLinkUpError).
//!
//! ```no_run
//! use libre_link_up_api_client::{LibreLinkUpClient, LibreLinkUpError};
//!
//! # async fn example() {
//! let client = LibreLinkUpClient::simple(
//!     "email@example.com".to_string(),
//!     "password".to_string(),
//!     None,
//! );
//!
//! match client {
//!     Ok(client) => match client.read().await {
//!         Ok(data) => println!("Success: {:.1} mg/dL", data.current.value),
//!         Err(LibreLinkUpError::NoConnections) => {
//!             eprintln!("No patients followed. Start following someone in the app.");
//!         }
//!         Err(e) => eprintln!("API error: {}", e),
//!     },
//!     Err(e) => eprintln!("Client error: {}", e),
//! }
//! # }
//! ```

pub mod client;
pub mod errors;
pub mod models;
pub mod utils;

pub use client::{ClientConfig, ConnectionIdentifier, LibreLinkUpClient};
pub use errors::{LibreLinkUpError, Result};
pub use models::{LibreCgmData, ReadRawResponse, ReadResponse, Region, TrendType};
