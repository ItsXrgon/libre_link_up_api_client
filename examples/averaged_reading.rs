//! Example of reading averaged glucose data over time

use libre_link_up_api_client::LibreLinkUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LibreLinkUpClient::simple(
        "your_email@example.com".to_string(),
        "your_password".to_string(),
        None, // Region will auto-detect
    )?;

    println!("Starting averaged reading (collecting 5 readings every 15 seconds)...");

    // Collect 5 readings and calculate average
    let handle = client
        .read_averaged(
            5,
            |average, memory, _history| {
                println!("\n=== Average Calculated ===");
                println!("Average value: {:.1} mg/dL", average.value);
                println!("Average trend: {:?}", average.trend);
                println!("Based on {} readings:", memory.len());
                for (i, reading) in memory.iter().enumerate() {
                    println!(
                        "  {}. {:.1} mg/dL at {}",
                        i + 1,
                        reading.value,
                        reading.date
                    );
                }
            },
            15000, // 15 second intervals
        )
        .await?;

    // Let it run for a while (this would normally run indefinitely)
    tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;

    // Cancel the background task
    handle.abort();

    Ok(())
}
