use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailRequest {
    pub from: String,
    pub to: String,
    pub subject: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_unsubscribe_footer: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_list_unsubscribe_header: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EmailAttachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<String>>,
}

/// An outgoing file. Set exactly one of `content` (base64) or `path` (public HTTPS URL).
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EmailAttachment {
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
}

impl EmailAttachment {
    /// Build an inline attachment from raw file bytes.
    pub fn from_bytes(
        filename: impl Into<String>,
        bytes: &[u8],
        content_type: impl Into<String>,
    ) -> Self {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        Self {
            filename: filename.into(),
            content: Some(STANDARD.encode(bytes)),
            path: None,
            content_type: Some(content_type.into()),
            content_id: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UploadedEmailAttachment {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub bucket: Option<String>,
    pub key: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SendMessageResponse {
    pub id: String,
}
