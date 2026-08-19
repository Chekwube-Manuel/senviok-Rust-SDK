use crate::{
    client::Senviok,
    error::SenviokError,
    models::{CreateTemplateRequest, ListPayload, Template, UpdateTemplateRequest},
};
use reqwest::Method;

pub struct Templates<'a> {
    pub(crate) client: &'a Senviok,
}

impl Templates<'_> {
    pub async fn create(&self, request: CreateTemplateRequest) -> Result<Template, SenviokError> {
        self.client
            .request(Method::POST, "/v1/templates", Some(&request), None::<&()>)
            .await
    }

    pub async fn list(&self) -> Result<Vec<Template>, SenviokError> {
        let payload: ListPayload<Template> = self
            .client
            .request(Method::GET, "/v1/templates", None::<&()>, None::<&()>)
            .await?;
        Ok(payload.into_vec())
    }

    pub async fn get(&self, id: &str) -> Result<Template, SenviokError> {
        let path = format!("/v1/templates/{id}");
        self.client
            .request(Method::GET, &path, None::<&()>, None::<&()>)
            .await
    }

    pub async fn update(
        &self,
        id: &str,
        request: UpdateTemplateRequest,
    ) -> Result<Template, SenviokError> {
        let path = format!("/v1/templates/{id}");
        self.client
            .request(Method::PUT, &path, Some(&request), None::<&()>)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<(), SenviokError> {
        let path = format!("/v1/templates/{id}");
        self.client
            .request_unit(Method::DELETE, &path, None::<&()>)
            .await
    }
}
