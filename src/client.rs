use crate::{
    error::SenviokError,
    models::ApiErrorResponse,
    resources::{
        audiences::Audiences, contacts::Contacts, domains::Domains, emails::Emails,
        messages::Messages, sms::Sms, suppressions::Suppressions, templates::Templates,
        webhooks::Webhooks, whatsapp::Whatsapp,
    },
};
use reqwest::Method;
use serde::{Serialize, de::DeserializeOwned};

const DEFAULT_BASE_URL: &str = "https://api.senviok.live";

/// Async Senviok API client.
///
/// Construct with [`Senviok::new`], then call a resource method such as
/// [`emails`](Self::emails) or [`sms`](Self::sms).
pub struct Senviok {
    api_key: String,
    base_url: String,
    http: reqwest::Client,
}

impl Senviok {
    /// Create a client that authenticates with `api_key` as a Bearer token.
    ///
    /// Requests go to `https://api.senviok.live` unless you chain
    /// [`with_base_url`](Self::with_base_url).
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Override the API host. Useful for staging.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Send transactional email.
    pub fn emails(&self) -> Emails<'_> {
        Emails { client: self }
    }

    /// Send SMS.
    pub fn sms(&self) -> Sms<'_> {
        Sms { client: self }
    }

    /// Send WhatsApp messages.
    pub fn whatsapp(&self) -> Whatsapp<'_> {
        Whatsapp { client: self }
    }

    /// Create and manage HTML email templates.
    pub fn templates(&self) -> Templates<'_> {
        Templates { client: self }
    }

    /// Register sending domains, fetch DKIM records, and verify DNS.
    pub fn domains(&self) -> Domains<'_> {
        Domains { client: self }
    }

    /// Create and list contact audiences.
    pub fn audiences(&self) -> Audiences<'_> {
        Audiences { client: self }
    }

    /// Manage contacts inside an audience.
    pub fn contacts(&self) -> Contacts<'_> {
        Contacts { client: self }
    }

    /// Block addresses from receiving further mail.
    pub fn suppressions(&self) -> Suppressions<'_> {
        Suppressions { client: self }
    }

    /// Query delivery logs.
    pub fn messages(&self) -> Messages<'_> {
        Messages { client: self }
    }

    /// Subscribe to delivery events.
    pub fn webhooks(&self) -> Webhooks<'_> {
        Webhooks { client: self }
    }

    pub(crate) async fn request<T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&impl Serialize>,
        query: Option<&impl Serialize>,
    ) -> Result<T, SenviokError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http.request(method, url).bearer_auth(&self.api_key);

        if let Some(query) = query {
            builder = builder.query(query);
        }

        if let Some(body) = body {
            builder = builder.json(body);
        }

        let response = builder.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            let message = serde_json::from_str::<ApiErrorResponse>(&text)
                .ok()
                .and_then(|error| error.message)
                .unwrap_or_else(|| {
                    if text.is_empty() {
                        "Unknown API error".to_string()
                    } else {
                        text.clone()
                    }
                });

            return Err(SenviokError::Api {
                status,
                message,
                body: text,
            });
        }

        if text.is_empty() {
            return serde_json::from_str("null").map_err(SenviokError::from);
        }

        match serde_json::from_str(&text) {
            Ok(value) => Ok(value),
            Err(error) => {
                if let Some(message) = serde_json::from_str::<ApiErrorResponse>(&text)
                    .ok()
                    .and_then(|body| body.message)
                {
                    Err(SenviokError::Api {
                        status,
                        message,
                        body: text,
                    })
                } else {
                    Err(error.into())
                }
            }
        }
    }

    pub(crate) async fn request_unit(
        &self,
        method: Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> Result<(), SenviokError> {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http.request(method, url).bearer_auth(&self.api_key);

        if let Some(body) = body {
            builder = builder.json(body);
        } else {
            builder = builder.json(&serde_json::json!({}));
        }

        let response = builder.send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            let message = serde_json::from_str::<ApiErrorResponse>(&text)
                .ok()
                .and_then(|error| error.message)
                .unwrap_or_else(|| {
                    if text.is_empty() {
                        "Unknown API error".to_string()
                    } else {
                        text.clone()
                    }
                });

            return Err(SenviokError::Api {
                status,
                message,
                body: text,
            });
        }

        Ok(())
    }
}
