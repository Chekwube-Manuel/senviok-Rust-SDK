# Senviok Rust SDK

Official async Rust client for the [Senviok](https://senviok.live) messaging API.

Send transactional **email**, **SMS**, and **WhatsApp** from one client, and manage the resources around them: templates, sending domains, audiences, contacts, suppressions, webhooks, and delivery logs.

```rust
let client = Senviok::new(std::env::var("SENVIOK_API_KEY")?);

let sent = client
    .emails()
    .send(SendEmailRequest {
        from: "Acme <onboarding@acme.com>".into(),
        to: "user@acme.com".into(),
        subject: "Welcome to Acme".into(),
        html: Some("<h1>Hello</h1><p>Welcome aboard.</p>".into()),
        text: Some("Hello — welcome aboard.".into()),
        ..Default::default()
    })
    .await?;

println!("queued {}", sent.id);
```

## Features

- Async `tokio` client over `reqwest`
- Bearer-token auth against `https://api.senviok.live`
- Typed request/response models with `serde`
- Structured errors for HTTP, decode, and API failures
- Override the base URL for staging or self-hosted environments

## Install

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
senviok = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

From a local checkout:

```toml
[dependencies]
senviok = { path = "../sendvoik-rust-client" }
```

Requires a recent stable Rust toolchain (this crate uses edition 2024).

## Authentication

Create an API key in the [Senviok dashboard](https://senviok.live), then pass it to `Senviok::new`. The client sends it as a Bearer token on every request.

```rust
use senviok::Senviok;

let client = Senviok::new("svk_live_...");
```

Keep keys out of source control. Copy [`.env.example`](.env.example) to `.env` for the examples; they load `SENVIOK_API_KEY` (and other vars) from the environment.

To point at a different API host:

```rust
let client = Senviok::new("svk_live_...")
    .with_base_url("https://api.staging.senviok.live");
```

## Quick start

```rust
use senviok::{SendEmailRequest, Senviok};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Senviok::new(std::env::var("SENVIOK_API_KEY")?);

    let res = client
        .emails()
        .send(SendEmailRequest {
            from: "onboarding@senviok.live".into(),
            to: "you@example.com".into(),
            subject: "Welcome to Acme!".into(),
            html: Some("<h1>Hello!</h1><p>Welcome aboard.</p>".into()),
            text: Some("Hello! Welcome aboard.".into()),
            ..Default::default()
        })
        .await?;

    println!("sent {}", res.id);
    Ok(())
}
```

Run the bundled example:

```bash
export SENVIOK_API_KEY=svk_live_...
cargo run --example send_email
```

## Resources

| Resource | Methods |
| --- | --- |
| `emails` | `send` |
| `sms` | `send` |
| `whatsapp` | `send` |
| `templates` | `create`, `list`, `get`, `update`, `delete` |
| `domains` | `create`, `list`, `get_dkim`, `verify` |
| `audiences` | `create`, `list`, `delete` |
| `contacts` | `create`, `list`, `delete` |
| `suppressions` | `create`, `list`, `delete` |
| `messages` | `list` |
| `webhooks` | `create`, `list`, `logs`, `delete` |

Every method returns `Result<T, SenviokError>`.

---

### Email

`SendEmailRequest` implements `Default`, so you only set the fields you need. Required: `from`, `to`, `subject`. Provide `html`, `text`, or a `template_id`.

```rust
use senviok::{SendEmailRequest, Senviok};
use serde_json::json;

let client = Senviok::new(api_key);

// HTML + plaintext
client
    .emails()
    .send(SendEmailRequest {
        from: "Acme <hello@acme.com>".into(),
        to: "user@acme.com".into(),
        subject: "Your invoice".into(),
        html: Some("<p>Invoice #1042 is ready.</p>".into()),
        text: Some("Invoice #1042 is ready.".into()),
        reply_to: Some("billing@acme.com".into()),
        cc: Some("ops@acme.com".into()),
        ..Default::default()
    })
    .await?;

// Template with merge data
client
    .emails()
    .send(SendEmailRequest {
        from: "hello@acme.com".into(),
        to: "user@acme.com".into(),
        subject: "Welcome".into(),
        template_id: Some("tmpl_123".into()),
        template_data: Some(json!({ "firstName": "Ada" })),
        add_unsubscribe_footer: Some(true),
        add_list_unsubscribe_header: Some(true),
        ..Default::default()
    })
    .await?;
```

Optional fields: `from_name`, `cc`, `bcc`, `reply_to`, `html`, `text`, `template_id`, `template_data`, `add_unsubscribe_footer`, `add_list_unsubscribe_header`.

---

### SMS

```rust
use senviok::{SendSmsRequest, Senviok};

client
    .sms()
    .send(SendSmsRequest {
        from: "Senviok".into(),
        to: "+2348012345678".into(),
        text: "Your code is 123456".into(),
    })
    .await?;
```

---

### WhatsApp

```rust
use senviok::{SendWhatsappRequest, Senviok};

client
    .whatsapp()
    .send(SendWhatsappRequest {
        from: "Senviok".into(),
        to: "+2348012345678".into(),
        text: "Your order has shipped.".into(),
    })
    .await?;
```

Email, SMS, and WhatsApp all return `SendMessageResponse { id }`.

---

### Templates

Reusable HTML email templates.

```rust
use senviok::{CreateTemplateRequest, UpdateTemplateRequest};

let template = client
    .templates()
    .create(CreateTemplateRequest {
        name: "Welcome".into(),
        subject: "Welcome to {{product}}".into(),
        html_content: "<h1>Hi {{firstName}}</h1>".into(),
    })
    .await?;

let listed = client.templates().list().await?;
let one = client.templates().get(&template.id).await?;

client
    .templates()
    .update(
        &template.id,
        UpdateTemplateRequest {
            subject: Some("You're in".into()),
            ..Default::default()
        },
    )
    .await?;

client.templates().delete(&template.id).await?;
```

---

### Domains

Register a sending domain, fetch DKIM records, then verify DNS.

```rust
use senviok::CreateDomainRequest;

let domain = client
    .domains()
    .create(CreateDomainRequest {
        name: "mail.acme.com".into(),
    })
    .await?;

let dkim = client.domains().get_dkim(&domain.id).await?;
let status = client.domains().verify(&domain.id).await?;
println!("{} verified={}", status.status, status.verified());

let all = client.domains().list().await?;
```

---

### Audiences & contacts

Audiences are lists. Contacts belong to an audience.

```rust
use senviok::{CreateAudienceRequest, CreateContactRequest};

let audience = client
    .audiences()
    .create(CreateAudienceRequest {
        name: "Launch waitlist".into(),
    })
    .await?;

let contact = client
    .contacts()
    .create(
        &audience.id,
        CreateContactRequest {
            email: "ada@example.com".into(),
            first_name: Some("Ada".into()),
            last_name: Some("Lovelace".into()),
            unsubscribed: Some(false),
        },
    )
    .await?;

let members = client.contacts().list(&audience.id).await?;
client.contacts().delete(&audience.id, &contact.id).await?;
client.audiences().delete(&audience.name).await?;
```

---

### Suppressions

Block an address from receiving further mail.

```rust
use senviok::CreateSuppressionRequest;

let entry = client
    .suppressions()
    .create(CreateSuppressionRequest {
        email: "bounce@example.com".into(),
        reason: "hard bounce".into(),
    })
    .await?;

let blocked = client.suppressions().list().await?;
client.suppressions().delete(&entry.id).await?;
```

---

### Message logs

Query delivery history with optional filters. All fields on `ListMessagesQuery` are optional.

```rust
use senviok::ListMessagesQuery;

let logs = client
    .messages()
    .list(ListMessagesQuery {
        take: Some(50),
        skip: Some(0),
        channel: Some("email".into()),
        status: Some("delivered".into()),
        to_address: Some("user@acme.com".into()),
        sort_order: Some("desc".into()),
        ..Default::default()
    })
    .await?;

for log in logs {
    println!("{} {} -> {:?}", log.created_at, log.status, log.to_address);
}
```

Filters: `take`, `skip`, `sort_order`, `channel`, `status`, `to_address`, `from_address`, `subject`, `start_date`, `end_date`.

---

### Webhooks

Subscribe to delivery events, inspect recent deliveries, then tear the endpoint down.

```rust
use senviok::CreateWebhookRequest;

let hook = client
    .webhooks()
    .create(CreateWebhookRequest {
        url: "https://api.acme.com/webhooks/senviok".into(),
        events: vec!["email.delivered".into(), "email.bounced".into()],
    })
    .await?;

println!("signing secret {}", hook.secret);

let endpoints = client.webhooks().list().await?;
let deliveries = client.webhooks().logs(&hook.id).await?;
client.webhooks().delete(&hook.id).await?;
```

Store `hook.secret` when the webhook is created — you need it to verify inbound signatures on your server.

## Error handling

All fallible calls return `SenviokError`:

```rust
use senviok::SenviokError;

match client.emails().send(request).await {
    Ok(sent) => println!("queued {}", sent.id),
    Err(SenviokError::Api { status, message, body }) => {
        eprintln!("API {status}: {message}");
        eprintln!("{body}");
    }
    Err(SenviokError::Request(err)) => eprintln!("transport: {err}"),
    Err(SenviokError::Decode(err)) => eprintln!("decode: {err}"),
}
```

| Variant | When |
| --- | --- |
| `Api { status, message, body }` | Non-success HTTP status. `message` is parsed from the JSON body when present; `body` is the raw response. |
| `Request` | Network / TLS / reqwest failure. |
| `Decode` | Response body was not valid JSON for the expected type. |

`SenviokError` implements `std::error::Error` via `thiserror`, so it works with `?` and `anyhow` / `eyre`.

## Client API

```rust
Senviok::new(api_key)
Senviok::with_base_url(self, base_url)  // builder; default https://api.senviok.live

client.emails()
client.sms()
client.whatsapp()
client.templates()
client.domains()
client.audiences()
client.contacts()
client.suppressions()
client.messages()
client.webhooks()
```

The crate re-exports the client, error type, and all request/response models from the crate root:

```rust
use senviok::{
    Audience, Contact, CreateAudienceRequest, CreateContactRequest, CreateDomainRequest,
    CreateSuppressionRequest, CreateTemplateRequest, CreateWebhookRequest, Domain,
    ListMessagesQuery, MessageLog, SendEmailRequest, SendMessageResponse, SendSmsRequest,
    SendWhatsappRequest, Senviok, SenviokError, Suppression, Template, UpdateTemplateRequest,
    Webhook,
};
```

## Examples

Copy [`.env.example`](.env.example) to `.env` and fill in your key (and any addresses you want to send to).

```bash
cp .env.example .env
cargo run --example send_email
```

| Example | What it does | Extra env |
| --- | --- | --- |
| [`send_email`](examples/send_email.rs) | Send a transactional email | `SENVIOK_FROM_EMAIL`, `SENVIOK_TO_EMAIL` |
| [`send_sms`](examples/send_sms.rs) | Send an SMS | `SENVIOK_FROM_SMS`, `SENVIOK_TO_PHONE` |
| [`send_whatsapp`](examples/send_whatsapp.rs) | Send a WhatsApp message | `SENVIOK_FROM_WHATSAPP` or `SENVIOK_FROM_SMS`, `SENVIOK_TO_PHONE` |
| [`templates`](examples/templates.rs) | Create, list, update, delete a template | — (cleans up after itself) |
| [`domains`](examples/domains.rs) | List domains; optionally create + DKIM + verify | `SENVIOK_DOMAIN` to mutate |
| [`audiences`](examples/audiences.rs) | Create an audience, add a contact, clean up | `SENVIOK_CONTACT_EMAIL` (optional) |
| [`suppressions`](examples/suppressions.rs) | Add, list, then remove a suppression | `SENVIOK_SUPPRESSION_EMAIL` (optional) |
| [`list_messages`](examples/list_messages.rs) | Print recent delivery logs | `SENVIOK_CHANNEL`, `SENVIOK_STATUS` (optional) |
| [`webhooks`](examples/webhooks.rs) | List webhooks; optionally create, inspect, delete | `SENVIOK_WEBHOOK_URL` to mutate |

`send_email`, `send_sms`, and `send_whatsapp` hit the live API and deliver a real message. The template / audience / suppression examples create records and delete them before exiting.

## Publishing to crates.io

This is a library crate. You do not deploy a server — you publish it so other Rust projects can depend on it with `cargo add senviok`.

**Before the first publish**

1. Create a [crates.io](https://crates.io) account (GitHub login) and run `cargo login`, pasting the API token from [crates.io/me](https://crates.io/me).
2. Push this repo to GitHub, then uncomment and set `repository` in `Cargo.toml`. crates.io and docs.rs use that URL.
3. Confirm the license. This repo ships MIT; change `LICENSE` and the `license` field if you need Apache-2.0 or dual-license.
4. Bump `version` in `Cargo.toml` for every release (semver: breaking API → `0.2.0` / `1.0.0`, compatible additions → `0.1.1`).

**Publish**

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo publish --dry-run
cargo publish
```

`cargo publish --dry-run` packs the crate and checks it without uploading. The real `cargo publish` is **permanent** for that version — you cannot overwrite `0.1.0`, only yank it.

After publish, docs build automatically at [docs.rs/senviok](https://docs.rs/senviok).

**GitHub Packages / private registries** are a different path (`cargo publish --registry`). For an official public SDK, crates.io is the one you want.

## Development

```bash
cargo build
cargo test --all-targets
cargo run --example send_email
```

CI (`.github/workflows/ci.yml`) runs rustfmt, clippy, tests, and rustdoc on push and pull request.

## License

MIT. See [LICENSE](LICENSE).
