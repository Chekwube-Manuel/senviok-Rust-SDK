use crate::{
    client::Senviok,
    error::SenviokError,
    models::{CreateSuppressionRequest, ListPayload, Suppression},
};
use reqwest::Method;

pub struct Suppressions<'a> {
    pub(crate) client: &'a Senviok,
}

impl Suppressions<'_> {
    pub async fn create(
        &self,
        request: CreateSuppressionRequest,
    ) -> Result<Suppression, SenviokError> {
        self.client
            .request(
                Method::POST,
                "/v1/suppressions",
                Some(&request),
                None::<&()>,
            )
            .await
    }

    pub async fn list(&self) -> Result<Vec<Suppression>, SenviokError> {
        let payload: ListPayload<Suppression> = self
            .client
            .request(Method::GET, "/v1/suppressions", None::<&()>, None::<&()>)
            .await?;
        Ok(payload.into_vec())
    }

    pub async fn delete(&self, id: &str) -> Result<(), SenviokError> {
        let path = format!("/v1/suppressions/{id}");
        self.client
            .request_unit(Method::DELETE, &path, None::<&()>)
            .await
    }
}
