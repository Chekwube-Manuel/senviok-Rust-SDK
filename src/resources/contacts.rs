use crate::{
    client::Senviok,
    error::SenviokError,
    models::{Contact, CreateContactRequest, ListPayload},
};
use reqwest::Method;

pub struct Contacts<'a> {
    pub(crate) client: &'a Senviok,
}

impl Contacts<'_> {
    pub async fn create(
        &self,
        audience_id: &str,
        request: CreateContactRequest,
    ) -> Result<Contact, SenviokError> {
        let path = format!("/v1/audiences/{audience_id}/contacts");
        self.client
            .request(Method::POST, &path, Some(&request), None::<&()>)
            .await
    }

    pub async fn list(&self, audience_id: &str) -> Result<Vec<Contact>, SenviokError> {
        let path = format!("/v1/audiences/{audience_id}/contacts");
        let payload: ListPayload<Contact> = self
            .client
            .request(Method::GET, &path, None::<&()>, None::<&()>)
            .await?;
        Ok(payload.into_vec())
    }

    pub async fn delete(&self, audience_id: &str, id: &str) -> Result<(), SenviokError> {
        let path = format!("/v1/audiences/{audience_id}/contacts/{id}");
        self.client
            .request_unit(Method::DELETE, &path, None::<&()>)
            .await
    }
}
