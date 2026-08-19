use crate::{
    client::Senviok,
    error::SenviokError,
    models::{SendMessageResponse, SendSmsRequest},
};
use reqwest::Method;

pub struct Sms<'a> {
    pub(crate) client: &'a Senviok,
}

impl Sms<'_> {
    pub async fn send(&self, request: SendSmsRequest) -> Result<SendMessageResponse, SenviokError> {
        self.client
            .request(Method::POST, "/v1/sms", Some(&request), None::<&()>)
            .await
    }
}
