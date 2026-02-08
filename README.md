# LibreLinkUp API Client (Rust)

Unofficial Rust client for the LibreLinkUp API - fetches CGM (Continuous Glucose Monitor) data from FreeStyle Libre 2/3 devices via Abbott's sharing service.

This is a Rust implementation inspired by [TypeScript libre-link-up-api-client](https://github.com/DiaKEM/libre-link-up-api-client) and [LibreLinkUp Status Bar Extension](https://github.com/borkod/librelinkup-vs-code-extension).

## Features

- ✅ Automatic authentication and token management
- ✅ Regional endpoint handling (US, EU, EU2, JP, DE, FR, AP, AU, AE)
- ✅ Read current and historical glucose data
- ✅ Raw API response access
- ✅ Averaged glucose readings over time
- ✅ Type-safe API with proper error handling
- ✅ Async/await support with Tokio

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
libre_link_up_api_client = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Simple Usage

```rust
use libre_link_up_api_client::LibreLinkUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with credentials
    let client = LibreLinkUpClient::simple(
        "your_email@example.com".to_string(),
        "your_password".to_string(),
    )?;

    // Read current glucose data
    let data = client.read().await?;

    println!("Current glucose: {:.1} mg/dL", data.current.value);
    println!("Trend: {:?}", data.current.trend);
    println!("Historical readings: {}", data.history.len());

    Ok(())
}
```

### Advanced Configuration

```rust
use libre_link_up_api_client::{ClientConfig, ConnectionIdentifier, LibreLinkUpClient};

let config = ClientConfig {
    username: "your_email@example.com".to_string(),
    password: "your_password".to_string(),
    client_version: Some("4.16.0".to_string()),
    connection_identifier: Some(ConnectionIdentifier::ByName("John Doe".to_string())),
};

let client = LibreLinkUpClient::new(config)?;
```

### Reading Raw Data

```rust
// Access raw API response with all details
let raw = client.read_raw().await?;

println!("Patient: {} {}", 
    raw.connection.first_name, 
    raw.connection.last_name
);
println!("Active sensors: {}", raw.active_sensors.len());
println!("Graph data points: {}", raw.graph_data.len());
```

### Averaged Readings

```rust
// Poll API and calculate averages over time
let handle = client.read_averaged(
    5,  // Collect 5 readings
    |average, memory, history| {
        println!("Average: {:.1} mg/dL", average.value);
        println!("Trend: {:?}", average.trend);
    },
    15000,  // Every 15 seconds
).await?;

// Runs in background until cancelled
handle.abort();
```

## API Methods

### `LibreLinkUpClient::simple(username, password)`
Create a basic client with default settings.

### `LibreLinkUpClient::new(config)`
Create a client with custom configuration.

### `client.read()`
Returns current glucose reading + historical data.

**Response:**
```rust
ReadResponse {
    current: LibreCgmData,  // Latest reading
    history: Vec<LibreCgmData>,  // Historical readings
}
```

### `client.read_raw()`
Returns raw API response with all details.

**Response:**
```rust
ReadRawResponse {
    connection: Connection,  // Patient connection info
    active_sensors: Vec<ActiveSensor>,  // Sensor details
    graph_data: Vec<GlucoseItem>,  // Raw glucose readings
}
```

### `client.read_averaged(amount, callback, interval_ms)`
Polls the API repeatedly and calculates averages.

**Parameters:**
- `amount`: Number of readings to collect before calculating average
- `callback`: Function called with (average, readings, history)
- `interval_ms`: Milliseconds between API calls

## Data Types

### `LibreCgmData`
```rust
pub struct LibreCgmData {
    pub value: f64,           // Glucose in mg/dL
    pub is_high: bool,        // Above target range
    pub is_low: bool,         // Below target range
    pub trend: TrendType,     // Arrow direction
    pub date: DateTime<Utc>,  // Timestamp
}
```

### `TrendType`
```rust
pub enum TrendType {
    SingleDown,      // ↓↓ Falling fast
    FortyFiveDown,   // ↘ Falling
    Flat,            // → Stable
    FortyFiveUp,     // ↗ Rising
    SingleUp,        // ↑↑ Rising fast
    NotComputable,   // ? Unknown
}
```

## Connection Identifiers

### By Name
```rust
ConnectionIdentifier::ByName("John Doe".to_string())
```

### By Custom Function
```rust
use std::sync::Arc;

ConnectionIdentifier::ByFunction(Arc::new(|connections| {
    connections.iter()
        .find(|c| c.patient_id == "12345")
        .map(|c| c.patient_id.clone())
}))
```

### None (Default)
Uses the first available connection.

## Error Handling

```rust
use libre_link_up_api_client::LibreLinkUpError;

match client.read().await {
    Ok(data) => println!("Glucose: {}", data.current.value),
    Err(LibreLinkUpError::BadCredentials) => {
        eprintln!("Invalid username or password");
    }
    Err(LibreLinkUpError::NoConnections) => {
        eprintln!("No patients being followed");
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Examples

Run the examples:

```bash
# Basic usage
cargo run --example basic_usage

# Averaged readings
cargo run --example averaged_reading
```

## Regional Support

The client automatically handles regional redirects. Supported regions:
- 🇺🇸 US (api-us.libreview.io)
- 🇪🇺 EU (api-eu.libreview.io)
- 🇪🇺 EU2 (api-eu2.libreview.io)
- 🇫🇷 FR (api-fr.libreview.io)
- 🇯🇵 JP (api-jp.libreview.io)
- 🇩🇪 DE (api-de.libreview.io)
- 🇦🇺 AU (api-au.libreview.io)
- 🇦🇪 AE (api-ae.libreview.io)
- 🌏 AP (Asia Pacific)

## License

MIT License

## Disclaimer

This is an unofficial client. Use at your own risk. The API is not officially documented and may change without notice.
