use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ListMessagesPayload {
    Wrapped { logs: Vec<MessageLog> },
    Bare(Vec<MessageLog>),
}

impl ListMessagesPayload {
    pub(crate) fn into_vec(self) -> Vec<MessageLog> {
        match self {
            Self::Wrapped { logs } => logs,
            Self::Bare(items) => items,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageLog {
    pub id: String,
    pub channel: String,
    pub status: String,
    pub to_address: Option<String>,
    pub from_address: Option<String>,
    pub subject: Option<String>,
    pub html_body: Option<String>,
    pub text_body: Option<String>,
    pub provider_message_id: Option<String>,
    pub created_at: String,
}
