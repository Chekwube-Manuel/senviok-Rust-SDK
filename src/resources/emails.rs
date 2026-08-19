use crate::{
    client::Senviok,
    error::SenviokError,
    models::{SendEmailRequest, SendMessageResponse},
};
use reqwest::Method;

pub struct Emails<'a> {
    pub(crate) client: &'a Senviok,
}

impl Emails<'_> {
    pub async fn send(
        &self,
        request: SendEmailRequest,
    ) -> Result<SendMessageResponse, SenviokError> {
        self.client
            .request(Method::POST, "/v1/emails", Some(&request), None::<&()>)
            .await
    }
}
