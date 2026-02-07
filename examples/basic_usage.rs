//! Example of using the LibreLinkUp API client
//!
//! Run with: cargo run --example basic_usage

use libre_link_up_api_client::{ClientConfig, ConnectionIdentifier, LibreLinkUpClient, Region};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple usage with just username and password
    let client = LibreLinkUpClient::simple(
        "your_email@example.com".to_string(),
        "your_password".to_string(),
        None, // Region will auto-detect
    )?;

    // Read current glucose data
    println!("Reading glucose data...");
    let data = client.read().await?;

    println!("Current glucose: {:.1} mg/dL", data.current.value);
    println!("Trend: {:?}", data.current.trend);
    println!("Is high: {}", data.current.is_high);
    println!("Is low: {}", data.current.is_low);
    println!("Date: {}", data.current.date);

    println!("\nHistorical readings: {} entries", data.history.len());

    // Advanced usage with configuration
    let config = ClientConfig {
        username: "your_email@example.com".to_string(),
        password: "your_password".to_string(),
        api_version: Some("4.12.0".to_string()),
        region: Some(Region::EU),
        connection_identifier: Some(ConnectionIdentifier::ByName("John Doe".to_string())),
    };

    let advanced_client = LibreLinkUpClient::new(config)?;

    // Read raw data
    let raw_data = advanced_client.read_raw().await?;
    println!(
        "\nConnection: {} {}",
        raw_data.connection.first_name, raw_data.connection.last_name
    );
    println!("Active sensors: {}", raw_data.active_sensors.len());

    Ok(())
}
