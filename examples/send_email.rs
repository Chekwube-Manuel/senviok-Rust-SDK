//! Send a transactional email.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... \
//! SENVIOK_FROM_EMAIL="Acme <hello@acme.com>" \
//! SENVIOK_TO_EMAIL=you@example.com \
//! cargo run --example send_email
//! ```

#[path = "common.rs"]
mod common;

use senviok::SendEmailRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let from = common::require_env("SENVIOK_FROM_EMAIL");
    let to = common::require_env("SENVIOK_TO_EMAIL");

    let sent = common::client()
        .emails()
        .send(SendEmailRequest {
            from,
            to,
            subject: "Welcome to Acme!".into(),
            html: Some("<h1>Hello!</h1><p>Welcome aboard.</p>".into()),
            text: Some("Hello! Welcome aboard.".into()),
            ..Default::default()
        })
        .await?;

    println!("queued email {}", sent.id);
    Ok(())
}
