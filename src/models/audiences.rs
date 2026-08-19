use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateAudienceRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Audience {
    pub id: String,
    pub name: String,
    pub created_at: String,
}
