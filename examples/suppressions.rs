//! Add an address to the suppression list, list entries, then remove it.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... cargo run --example suppressions
//! ```

#[path = "common.rs"]
mod common;

use senviok::CreateSuppressionRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = common::client();
    let email = common::optional_env("SENVIOK_SUPPRESSION_EMAIL")
        .unwrap_or_else(|| "sdk-example-bounce@example.com".into());

    let entry = client
        .suppressions()
        .create(CreateSuppressionRequest {
            email,
            reason: "sdk example — hard bounce".into(),
        })
        .await?;
    println!("suppressed {} ({})", entry.email, entry.id);

    let listed = client.suppressions().list().await?;
    println!("account has {} suppression(s)", listed.len());

    client.suppressions().delete(&entry.id).await?;
    println!("removed suppression {}", entry.id);
    Ok(())
}
