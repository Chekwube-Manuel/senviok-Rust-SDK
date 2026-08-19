//! Send an SMS.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... \
//! SENVIOK_FROM_SMS=Senviok \
//! SENVIOK_TO_PHONE=+2348012345678 \
//! cargo run --example send_sms
//! ```

#[path = "common.rs"]
mod common;

use senviok::SendSmsRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let from = common::require_env("SENVIOK_FROM_SMS");
    let to = common::require_env("SENVIOK_TO_PHONE");

    let sent = common::client()
        .sms()
        .send(SendSmsRequest {
            from,
            to,
            text: "Your Senviok verification code is 123456".into(),
        })
        .await?;

    println!("queued sms {}", sent.id);
    Ok(())
}
