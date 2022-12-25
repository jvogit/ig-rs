use crate::igclient::{IGClient, Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub mod accounts_login;
pub mod qe_sync;

#[async_trait]
pub trait IGRequest<Req, Res>
where
    Req: Serialize,
    Res: DeserializeOwned,
{
    fn url(&self) -> String;
    fn payload(&self) -> &Req;
    async fn send(&self, client: &IGClient) -> Result<Res>;
}

pub trait IGPostRequest<Req, Res>
where
    Req: Serialize,
    Res: DeserializeOwned,
{
    fn payload(&self) -> &Req;
    fn url(&self) -> String;
}

#[async_trait]
impl<T: IGPostRequest<Req, Res>, Req, Res> IGRequest<Req, Res> for T
where
    Self: Serialize + Sync,
    Req: Serialize,
    Res: DeserializeOwned,
{
    fn payload(&self) -> &Req {
        self.payload()
    }

    fn url(&self) -> String {
        self.url()
    }

    async fn send(&self, client: &IGClient) -> Result<Res> {
        Ok(client.send(self).await?)
    }
}

#[derive(Serialize)]
pub struct IGRequestMetadata {
    #[serde(rename = "_csrftoken")]
    pub csrftoken: String,
    pub id: String,
    pub guid: String,
    pub device_id: String,
    pub phone_id: String,
}

impl IGRequestMetadata {
    pub async fn from_client(client: &IGClient) -> Self {
        let ig_client_config = client.ig_client_config.read().await;

        IGRequestMetadata {
            csrftoken: ig_client_config.csrftoken.to_string(),
            id: ig_client_config.guid.to_string(),
            guid: ig_client_config.guid.to_string(),
            device_id: ig_client_config.device.device_id.to_string(),
            phone_id: ig_client_config.guid.to_string(),
        }
    }
}
