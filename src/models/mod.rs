mod audiences;
mod contacts;
mod domains;
mod emails;
mod messages;
mod sms;
mod suppressions;
mod templates;
mod webhooks;
mod whatsapp;

pub use audiences::*;
pub use contacts::*;
pub use domains::*;
pub use emails::*;
pub use messages::*;
pub use sms::*;
pub use suppressions::*;
pub use templates::*;
pub use webhooks::*;
pub use whatsapp::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorResponse {
    #[serde(alias = "Message")]
    pub message: Option<String>,
}

/// Some list endpoints return a bare array; others wrap it as `{ "data": [...] }`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ListPayload<T> {
    Wrapped { data: Vec<T> },
    Bare(Vec<T>),
}

impl<T> ListPayload<T> {
    pub(crate) fn into_vec(self) -> Vec<T> {
        match self {
            Self::Wrapped { data } => data,
            Self::Bare(items) => items,
        }
    }
}
