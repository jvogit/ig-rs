use std::collections::HashMap;

use crate::igclient::IGClientConfig;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub mod accounts_login;
pub mod qe_sync;

#[async_trait]
pub trait IGRequest<Body>
where
    Body: DeserializeOwned,
{
    async fn send(
        &self,
        client: &reqwest::Client,
        ig_client_config: &mut IGClientConfig,
    ) -> Result<Body, reqwest::Error>;
}

pub trait IGPostRequest<Res>
where
    Res: DeserializeOwned,
{
    fn url(&self) -> String;
}

#[async_trait]
impl<T: IGPostRequest<Body>, Body> IGRequest<Body> for T
where
    Self: Serialize + Sync,
    Body: DeserializeOwned,
{
    async fn send(
        &self,
        client: &reqwest::Client,
        ig_client_config: &mut IGClientConfig,
    ) -> Result<Body, reqwest::Error> {
        let payload = serde_json::to_string(self).expect("body to be able to serialize to JSON");

        println!("Payload {:#?}", payload);

        let mut params = HashMap::new();
        // Instagram POST payloads do not require actual signature anymore. Now replaced with "SIGNATURE".
        params.insert("signed_body", format!("SIGNATURE.{payload}"));

        let request = client
            .post(self.url())
            .header("Connection", "close")
            .header("X-IG-Capabilities", &ig_client_config.device.capabilities)
            .header("X-IG-App-ID", "567067343352427")
            .header("User-Agent", &ig_client_config.device.user_agent)
            .header("X-IG-Device-ID", &ig_client_config.guid)
            .header("X-IG-Android-ID", &ig_client_config.device.device_id)
            .form(&params)
            .build()?;
        println!("Request {:#?}", request);
        let response = client.execute(request).await?;
        println!("Response {:#?}", response);
        if let Some(csrftoken) = get_cookie_value(&response, "csrftoken") {
            ig_client_config.csrftoken = csrftoken;
        }
        let body = response.json::<Body>().await?;

        Ok(body)
    }
}

pub fn get_cookie_value(response: &reqwest::Response, cookie_name: &str) -> Option<String> {
    if let Some(cookie) = response
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .map(|hv| cookie::Cookie::parse(hv.to_str().unwrap()).unwrap())
        .find(|cookie| cookie.name() == cookie_name)
    {
        return Some(cookie.value().to_string());
    }
    None
}

#[derive(Serialize)]
pub struct IGLoggedOutRequestMetadata {
    #[serde(rename = "_csrftoken")]
    pub csrftoken: String,
    pub id: String,
    pub guid: String,
    pub device_id: String,
    pub phone_id: String,
}

impl From<&IGClientConfig> for IGLoggedOutRequestMetadata {
    fn from(value: &IGClientConfig) -> Self {
        IGLoggedOutRequestMetadata {
            csrftoken: value.csrftoken.to_string(),
            id: value.guid.to_string(),
            guid: value.guid.to_string(),
            device_id: value.device.device_id.to_string(),
            phone_id: value.guid.to_string(),
        }
    }
}
