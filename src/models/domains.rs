use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct CreateDomainRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DkimResponse {
    pub tokens: Vec<Value>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VerifyDomainResponse {
    pub status: String,
    #[serde(default)]
    pub diagnostics: Option<VerifyDomainDiagnostics>,
}

impl VerifyDomainResponse {
    pub fn verified(&self) -> bool {
        self.status.eq_ignore_ascii_case("verified")
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct VerifyDomainDiagnostics {
    pub checked: bool,
    pub records: Vec<DnsRecordDiagnostic>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecordDiagnostic {
    pub record_type: String,
    pub host: String,
    pub expected_value: String,
    pub status: String,
    pub actual_value: Option<String>,
    pub message: Option<String>,
    pub provider_status: Option<bool>,
}
