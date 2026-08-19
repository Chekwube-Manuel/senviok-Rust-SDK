//! Official async Rust client for the [Senviok](https://senviok.live) messaging API.
//!
//! Send transactional email, SMS, and WhatsApp, and manage templates, domains,
//! audiences, contacts, suppressions, webhooks, and delivery logs.
//!
//! ```no_run
//! use senviok::{SendEmailRequest, Senviok};
//!
//! # async fn run() -> Result<(), senviok::SenviokError> {
//! let client = Senviok::new("svk_live_...");
//!
//! let sent = client
//!     .emails()
//!     .send(SendEmailRequest {
//!         from: "Acme <hello@acme.com>".into(),
//!         to: "user@acme.com".into(),
//!         subject: "Welcome".into(),
//!         html: Some("<p>Hello</p>".into()),
//!         ..Default::default()
//!     })
//!     .await?;
//!
//! println!("{}", sent.id);
//! # Ok(())
//! # }
//! ```
//!
//! See the crate README for full examples. The default API host is
//! `https://api.senviok.live`.

pub mod client;
pub mod error;
pub mod models;
pub mod resources;

pub use client::Senviok;
pub use error::SenviokError;
pub use models::*;
