use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: String,
    pub url: String,
    pub secret: String,
    pub events: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct WebhookDelivery {
    #[serde(flatten)]
    pub data: Value,
}
