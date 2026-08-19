use crate::{
    client::Senviok,
    error::SenviokError,
    models::{ListMessagesPayload, ListMessagesQuery, MessageLog},
};
use reqwest::Method;

pub struct Messages<'a> {
    pub(crate) client: &'a Senviok,
}

impl Messages<'_> {
    pub async fn list(&self, query: ListMessagesQuery) -> Result<Vec<MessageLog>, SenviokError> {
        let payload: ListMessagesPayload = self
            .client
            .request(Method::GET, "/v1/analytics/logs", None::<&()>, Some(&query))
            .await?;
        Ok(payload.into_vec())
    }
}
