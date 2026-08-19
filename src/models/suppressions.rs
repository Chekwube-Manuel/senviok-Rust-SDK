use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateSuppressionRequest {
    pub email: String,
    pub reason: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Suppression {
    pub id: String,
    pub email: String,
    pub reason: String,
    pub created_at: String,
}
