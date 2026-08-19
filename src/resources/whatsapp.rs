use crate::{
    client::Senviok,
    error::SenviokError,
    models::{SendMessageResponse, SendWhatsappRequest},
};
use reqwest::Method;

pub struct Whatsapp<'a> {
    pub(crate) client: &'a Senviok,
}

impl Whatsapp<'_> {
    pub async fn send(
        &self,
        request: SendWhatsappRequest,
    ) -> Result<SendMessageResponse, SenviokError> {
        self.client
            .request(Method::POST, "/v1/whatsapp", Some(&request), None::<&()>)
            .await
    }
}
