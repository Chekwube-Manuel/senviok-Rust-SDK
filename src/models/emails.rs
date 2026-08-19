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
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SendMessageResponse {
    pub id: String,
}
