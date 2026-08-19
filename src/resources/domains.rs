use crate::{
    client::Senviok,
    error::SenviokError,
    models::{CreateDomainRequest, DkimResponse, Domain, ListPayload, VerifyDomainResponse},
};
use reqwest::Method;

pub struct Domains<'a> {
    pub(crate) client: &'a Senviok,
}

impl Domains<'_> {
    pub async fn create(&self, request: CreateDomainRequest) -> Result<Domain, SenviokError> {
        self.client
            .request(Method::POST, "/v1/domains", Some(&request), None::<&()>)
            .await
    }

    pub async fn list(&self) -> Result<Vec<Domain>, SenviokError> {
        let payload: ListPayload<Domain> = self
            .client
            .request(Method::GET, "/v1/domains", None::<&()>, None::<&()>)
            .await?;
        Ok(payload.into_vec())
    }

    pub async fn get_dkim(&self, id: &str) -> Result<DkimResponse, SenviokError> {
        let path = format!("/v1/domains/{id}/dkim");
        self.client
            .request(Method::GET, &path, None::<&()>, None::<&()>)
            .await
    }

    pub async fn verify(&self, id: &str) -> Result<VerifyDomainResponse, SenviokError> {
        let path = format!("/v1/domains/{id}/verify");
        self.client
            .request(Method::POST, &path, None::<&()>, None::<&()>)
            .await
    }
}
