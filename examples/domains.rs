//! List sending domains. Optionally register and verify one.
//!
//! ```bash
//! SENVIOK_API_KEY=svk_live_... cargo run --example domains
//!
//! # also create + fetch DKIM + verify:
//! SENVIOK_DOMAIN=mail.acme.com cargo run --example domains
//! ```

#[path = "common.rs"]
mod common;

use senviok::CreateDomainRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = common::client();

    let domains = client.domains().list().await?;
    println!("account has {} domain(s)", domains.len());
    for domain in &domains {
        println!("  {}  {}  {}", domain.id, domain.status, domain.name);
    }

    let Some(name) = common::optional_env("SENVIOK_DOMAIN") else {
        println!("set SENVIOK_DOMAIN to create, fetch DKIM, and verify a domain");
        return Ok(());
    };

    let domain = client
        .domains()
        .create(CreateDomainRequest { name })
        .await?;
    println!("created domain {} ({})", domain.id, domain.name);

    let dkim = client.domains().get_dkim(&domain.id).await?;
    println!(
        "dkim records: {}",
        serde_json::to_string_pretty(&dkim.tokens)?
    );

    let verified = client.domains().verify(&domain.id).await?;
    println!(
        "status={} verified={}",
        verified.status,
        verified.verified()
    );
    Ok(())
}
