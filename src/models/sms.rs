use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SendSmsRequest {
    pub from: String,
    pub to: String,
    pub text: String,
}
