use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SendWhatsappRequest {
    pub from: String,
    pub to: String,
    pub text: String,
}
