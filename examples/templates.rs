//! Create, list, update, and delete an email template. Cleans up after itself.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... cargo run --example templates
//! ```

#[path = "common.rs"]
mod common;

use senviok::{CreateTemplateRequest, UpdateTemplateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = common::client();

    let created = client
        .templates()
        .create(CreateTemplateRequest {
            name: "sdk-example-welcome".into(),
            subject: "Welcome to {{product}}".into(),
            html_content: "<h1>Hi {{firstName}}</h1><p>Thanks for joining.</p>".into(),
        })
        .await?;
    println!("created template {}", created.id);

    let listed = client.templates().list().await?;
    println!("account has {} template(s)", listed.len());

    let fetched = client.templates().get(&created.id).await?;
    println!("fetched {} ({})", fetched.name, fetched.subject);

    let updated = client
        .templates()
        .update(
            &created.id,
            UpdateTemplateRequest {
                name: Some(created.name.clone()),
                subject: Some("You're in — welcome to {{product}}".into()),
                html_content: Some(created.html_content.clone()),
            },
        )
        .await?;
    println!("updated subject to {}", updated.subject);

    client.templates().delete(&created.id).await?;
    println!("deleted template {}", created.id);
    Ok(())
}
