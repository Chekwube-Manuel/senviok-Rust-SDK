use crate::{
    client::Senviok,
    error::SenviokError,
    models::{Audience, CreateAudienceRequest, ListPayload},
};
use reqwest::Method;

pub struct Audiences<'a> {
    pub(crate) client: &'a Senviok,
}

impl Audiences<'_> {
    pub async fn create(&self, request: CreateAudienceRequest) -> Result<Audience, SenviokError> {
        self.client
            .request(Method::POST, "/v1/audiences", Some(&request), None::<&()>)
            .await
    }

    pub async fn list(&self) -> Result<Vec<Audience>, SenviokError> {
        let payload: ListPayload<Audience> = self
            .client
            .request(Method::GET, "/v1/audiences", None::<&()>, None::<&()>)
            .await?;
        Ok(payload.into_vec())
    }

    pub async fn delete(&self, name: &str) -> Result<(), SenviokError> {
        let path = format!("/v1/audiences/{name}");
        self.client
            .request_unit(Method::DELETE, &path, None::<&()>)
            .await
    }
}
