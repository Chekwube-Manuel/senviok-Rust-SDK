//! Create an audience, add a contact, then clean up.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... cargo run --example audiences
//!
//! # optional: SENVIOK_CONTACT_EMAIL=ada@example.com
//! ```

#[path = "common.rs"]
mod common;

use senviok::{CreateAudienceRequest, CreateContactRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = common::client();
    let email = common::optional_env("SENVIOK_CONTACT_EMAIL")
        .unwrap_or_else(|| "sdk-example@example.com".into());

    let audience = client
        .audiences()
        .create(CreateAudienceRequest {
            name: "sdk-example-waitlist".into(),
        })
        .await?;
    println!("created audience {}", audience.id);

    let contact = client
        .contacts()
        .create(
            &audience.id,
            CreateContactRequest {
                email,
                first_name: Some("Ada".into()),
                last_name: Some("Lovelace".into()),
                unsubscribed: Some(false),
            },
        )
        .await?;
    println!("added contact {} ({})", contact.id, contact.email);

    let members = client.contacts().list(&audience.id).await?;
    println!("audience has {} contact(s)", members.len());

    client.contacts().delete(&audience.id, &contact.id).await?;
    client.audiences().delete(&audience.name).await?;
    println!("cleaned up audience {}", audience.id);
    Ok(())
}
