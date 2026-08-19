//! Query recent delivery logs.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... cargo run --example list_messages
//!
//! # optional filters:
//! SENVIOK_CHANNEL=email SENVIOK_STATUS=delivered cargo run --example list_messages
//! ```

#[path = "common.rs"]
mod common;

use senviok::ListMessagesQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logs = common::client()
        .messages()
        .list(ListMessagesQuery {
            take: Some(10),
            skip: Some(0),
            sort_order: Some("desc".into()),
            channel: common::optional_env("SENVIOK_CHANNEL"),
            status: common::optional_env("SENVIOK_STATUS"),
            to_address: common::optional_env("SENVIOK_TO_EMAIL"),
            ..Default::default()
        })
        .await?;

    println!("{} log(s)", logs.len());
    for log in logs {
        println!(
            "  {}  {:<10} {:<12} {} -> {}",
            log.created_at,
            log.channel,
            log.status,
            log.from_address.as_deref().unwrap_or("-"),
            log.to_address.as_deref().unwrap_or("-"),
        );
    }
    Ok(())
}
