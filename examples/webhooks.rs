//! List webhook endpoints. Optionally create one, print deliveries, then delete it.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... cargo run --example webhooks
//!
//! # also create + inspect + delete:
//! SENVIOK_WEBHOOK_URL=https://example.com/webhooks/senviok cargo run --example webhooks
//! ```

#[path = "common.rs"]
mod common;

use senviok::CreateWebhookRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = common::client();

    let hooks = client.webhooks().list().await?;
    println!("account has {} webhook(s)", hooks.len());
    for hook in &hooks {
        println!("  {}  {}  {:?}", hook.id, hook.url, hook.events);
    }

    let Some(url) = common::optional_env("SENVIOK_WEBHOOK_URL") else {
        println!("set SENVIOK_WEBHOOK_URL to create a webhook, print its secret, then delete it");
        return Ok(());
    };

    let hook = client
        .webhooks()
        .create(CreateWebhookRequest {
            url,
            events: vec!["email.delivered".into(), "email.bounced".into()],
        })
        .await?;
    println!("created webhook {}", hook.id);
    println!("signing secret: {}", hook.secret);

    let deliveries = client.webhooks().logs(&hook.id).await?;
    println!("{} delivery attempt(s)", deliveries.len());

    client.webhooks().delete(&hook.id).await?;
    println!("deleted webhook {}", hook.id);
    Ok(())
}
