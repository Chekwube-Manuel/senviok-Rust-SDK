use crate::{
    client::Senviok,
    error::SenviokError,
    models::{CreateWebhookRequest, ListPayload, Webhook, WebhookDelivery},
};
use reqwest::Method;

pub struct Webhooks<'a> {
    pub(crate) client: &'a Senviok,
}

impl Webhooks<'_> {
    pub async fn create(&self, request: CreateWebhookRequest) -> Result<Webhook, SenviokError> {
        self.client
            .request(Method::POST, "/v1/webhooks", Some(&request), None::<&()>)
            .await
    }

    pub async fn list(&self) -> Result<Vec<Webhook>, SenviokError> {
        let payload: ListPayload<Webhook> = self
            .client
            .request(Method::GET, "/v1/webhooks", None::<&()>, None::<&()>)
            .await?;
        Ok(payload.into_vec())
    }

    pub async fn logs(&self, id: &str) -> Result<Vec<WebhookDelivery>, SenviokError> {
        let path = format!("/v1/webhooks/{id}/deliveries");
        let payload: ListPayload<WebhookDelivery> = self
            .client
            .request(Method::GET, &path, None::<&()>, None::<&()>)
            .await?;
        Ok(payload.into_vec())
    }

    pub async fn delete(&self, id: &str) -> Result<(), SenviokError> {
        let path = format!("/v1/webhooks/{id}");
        self.client
            .request_unit(Method::DELETE, &path, None::<&()>)
            .await
    }
}
