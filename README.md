# libre_link_up_api_client

Unofficial Rust client for the LibreLinkUp API. Fetches CGM data from FreeStyle Libre 2/3 devices via Abbott's sharing service.

Inspired by [TypeScript libre-link-up-api-client](https://github.com/DiaKEM/libre-link-up-api-client) and [LibreLinkUp Status Bar Extension](https://github.com/borkod/librelinkup-vs-code-extension).

## Installation

```toml
[dependencies]
libre_link_up_api_client = "0.2.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Example

```rust
use libre_link_up_api_client::LibreLinkUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LibreLinkUpClient::simple(
        "your_email@example.com".to_string(),
        "your_password".to_string(),
        None,
    )?;

    let data = client.read().await?;
    println!("Glucose: {:.1} mg/dL, trend: {:?}", data.current.value, data.current.trend);
    Ok(())
}
```

## Documentation

- **API docs**: [docs.rs/libre_link_up_api_client](https://docs.rs/libre_link_up_api_client) or `cargo doc --open`
- **Libre APIs** (external): [LibreView Unofficial API](https://libreview-unofficial.stoplight.io/docs/libreview-unofficial/8i2x0tc4qumh2-authentication); see also [Gist with resources](https://gist.github.com/khskekec/6c13ba01b10d3018d816706a32ae8ab2?permalink_comment_id=5330300)

## Features

- Authentication and token handling
- Regional endpoints (US, EU, JP, DE, FR, AP, AU, AE, CA, LA, RU, CN, etc.)
- Glucose readings (current, history, raw, averaged)
- User, account, logbook, notification settings, country config

## Examples

```bash
cargo run --example basic_usage
cargo run --example averaged_reading
```

## License

MIT

## Disclaimer

Unofficial client. The API is not officially documented and may change. Use at your own risk.
