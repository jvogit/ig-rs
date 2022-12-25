use self::{
    igdevice::IGAndroidDevice,
    igrequests::{accounts_login::LoginResponse, IGRequest},
};
use reqwest::{
    cookie::{CookieStore, Jar},
    Client, Url,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

const BASE_IG_API_V1: &str = "https://i.instagram.com/api/v1/";

pub mod igconstants;
pub mod igdevice;
pub mod igrequests;

#[derive(Debug)]
pub enum IGClientErr {
    HTTPClientError(reqwest::Error),
    IGLoginError(IGLoginErrorResponse),
}

impl From<reqwest::Error> for IGClientErr {
    fn from(error: reqwest::Error) -> Self {
        return IGClientErr::HTTPClientError(error);
    }
}

#[derive(Debug)]
pub enum IGLoginErrorResponse {
    QeSyncResponse(igrequests::qe_sync::QeResponse),
    AccountsLoginResponse(igrequests::accounts_login::LoginResponse),
}

pub type Result<T> = std::result::Result<T, IGClientErr>;

pub struct IGClient {
    client: Client,
    cookie_store: Arc<Jar>,
    ig_client_config: Arc<RwLock<IGClientConfig>>,
}

impl IGClient {
    pub fn new() -> Self {
        let cookie_store = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            .build()
            .unwrap();
        let ig_client_config = IGClientConfig {
            guid: Uuid::new_v4().to_string(),
            device: IGAndroidDevice::new("1234"),
            csrftoken: "missing".to_string(),
            cookies_str: "".to_string(),
        };

        IGClient {
            client,
            cookie_store,
            ig_client_config: Arc::new(RwLock::new(ig_client_config)),
        }
    }
    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        let qe_sync_response = igrequests::qe_sync::QeRequest {
            metadata: igrequests::IGRequestMetadata::from_client(self).await,
            experiments: igconstants::IG_EXPERIMENTS.to_string(),
        }
        .send(self)
        .await?;

        if let igrequests::qe_sync::QeResponse::Fail { .. } = qe_sync_response {
            return Err(IGClientErr::IGLoginError(
                IGLoginErrorResponse::QeSyncResponse(qe_sync_response),
            ));
        }

        let login_response = igrequests::accounts_login::LoginRequest {
            metadata: igrequests::IGRequestMetadata::from_client(self).await,
            username: username.to_string(),
            enc_password: format!("#PWD_INSTAGRAM:0:&:{password}"),
            login_attempt_account: 0,
        }
        .send(self)
        .await?;

        if let igrequests::accounts_login::LoginResponse::Fail { .. } = login_response {
            return Err(IGClientErr::IGLoginError(
                IGLoginErrorResponse::AccountsLoginResponse(login_response),
            ));
        }

        Ok(login_response)
    }

    pub async fn with_ig_client_config(ig_client_config: IGClientConfig) -> Result<IGClient> {
        let cookie_store = Arc::new(Jar::default());
        cookie_store.add_cookie_str(
            &ig_client_config.cookies_str,
            &BASE_IG_API_V1.parse::<Url>().unwrap(),
        );
        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            .build()
            .unwrap();

        Ok(IGClient {
            client,
            cookie_store,
            ig_client_config: Arc::new(RwLock::new(ig_client_config)),
        })
    }

    pub async fn send<Req, Res>(&self, ig_request: &(dyn IGRequest<Req, Res> + Sync)) -> Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        let payload = serde_json::to_string(ig_request.payload())
            .expect("body to be able to serialize to JSON");

        // TODO: Replace with log
        // println!("Payload {:#?}", payload);

        let mut params = HashMap::new();
        // Instagram POST payloads do not require actual signature anymore. Now replaced with "SIGNATURE".
        params.insert("signed_body", format!("SIGNATURE.{payload}"));
        let ig_client_config = self.ig_client_config.read().await;
        let request = self
            .client
            .post(ig_request.url())
            .header("Connection", "close")
            .header("X-IG-Capabilities", &ig_client_config.device.capabilities)
            .header("X-IG-App-ID", "567067343352427")
            .header("User-Agent", &ig_client_config.device.user_agent)
            .header("X-IG-Device-ID", &ig_client_config.guid)
            .header("X-IG-Android-ID", &ig_client_config.device.device_id)
            .form(&params)
            .build()?;
        drop(ig_client_config);

        // TODO: Replace with log
        // println!("Request {:#?}", request);
        let response = self.client.execute(request).await?;
        // TODO: Replace with log
        // println!("Response {:#?}", response);
        let mut ig_client_config = self.ig_client_config.write().await;
        if let Some(csrftoken) = get_set_cookie_value(&response, "csrftoken") {
            ig_client_config.csrftoken = csrftoken;
        }
        if let Some(session_cookies) = self.session_cookies() {
            ig_client_config.cookies_str = session_cookies;
        }

        let body = response.json::<Res>().await?;

        Ok(body)
    }

    pub async fn ig_client_config(&self) -> IGClientConfig {
        self.ig_client_config.read().await.clone()
    }

    pub fn blocking_ig_client_config(&self) -> IGClientConfig {
        self.ig_client_config.blocking_read().clone()
    }

    fn session_cookies(&self) -> Option<String> {
        self.cookie_store
            .cookies(&BASE_IG_API_V1.parse::<Url>().unwrap())
            .map(|hv| hv.to_str().unwrap().to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IGClientConfig {
    pub guid: String,
    pub device: IGAndroidDevice,
    pub csrftoken: String,
    pub cookies_str: String,
}

impl IGClientConfig {
    pub fn get_cookie_value(&self, name: &str) -> Option<String> {
        self.cookies_str
            .split("; ")
            .map(|s| cookie::Cookie::parse(s).expect("Cookie to be parsed"))
            .find(|cookie| cookie.name() == name)
            .map(|cookie| cookie.value().to_string())
    }
}

fn get_set_cookie_value(response: &reqwest::Response, cookie_name: &str) -> Option<String> {
    response
        .headers()
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .map(|hv| cookie::Cookie::parse(hv.to_str().unwrap()).unwrap())
        .find(|cookie| cookie.name() == cookie_name)
        .map(|cookie| cookie.value().to_string())
}
