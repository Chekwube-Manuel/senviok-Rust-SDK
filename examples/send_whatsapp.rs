//! Send a WhatsApp message.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... \
//! SENVIOK_FROM_WHATSAPP=Senviok \
//! SENVIOK_TO_PHONE=+2348012345678 \
//! cargo run --example send_whatsapp
//! ```

#[path = "common.rs"]
mod common;

use senviok::SendWhatsappRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let from = common::optional_env("SENVIOK_FROM_WHATSAPP")
        .unwrap_or_else(|| common::require_env("SENVIOK_FROM_SMS"));
    let to = common::require_env("SENVIOK_TO_PHONE");

    let sent = common::client()
        .whatsapp()
        .send(SendWhatsappRequest {
            from,
            to,
            text: "Your order has shipped. Track it in the Acme app.".into(),
        })
        .await?;

    println!("queued whatsapp {}", sent.id);
    Ok(())
}
