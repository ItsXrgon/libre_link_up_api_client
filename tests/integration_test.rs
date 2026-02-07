//! Integration tests for LibreLinkUp API client
//!
//! Note: These tests require valid credentials to run against the real API.
//! They are disabled by default to avoid rate limiting.

#[cfg(test)]
mod integration {
    use libre_link_up_api_client::{ClientConfig, LibreLinkUpClient};

    // Helper to check if test credentials are available
    fn has_test_credentials() -> bool {
        // Try to load .env file (ignore if it doesn't exist)
        let _ = dotenvy::dotenv();

        std::env::var("LIBRE_LINK_EMAIL").is_ok() && std::env::var("LIBRE_LINK_PASSWORD").is_ok()
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_client_creation() {
        let config = ClientConfig {
            username: "test@example.com".to_string(),
            password: "test".to_string(),
            api_version: None,
            region: None,
            connection_identifier: None,
        };

        let result = LibreLinkUpClient::new(config);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_read_with_real_credentials() {
        if !has_test_credentials() {
            println!("Skipping: Set LIBRE_LINK_EMAIL and LIBRE_LINK_PASSWORD to run");
            return;
        }
        let email = std::env::var("LIBRE_LINK_EMAIL").unwrap();
        let password = std::env::var("LIBRE_LINK_PASSWORD").unwrap();

        let client = LibreLinkUpClient::simple(email, password, None).unwrap();
        let result = client.read().await;

        match result {
            Ok(data) => {
                assert!(data.current.value > 0.0);
                assert!(!data.history.is_empty());
            }
            Err(e) => {
                panic!("API call failed: {:?}", e);
            }
        }
    }
}
