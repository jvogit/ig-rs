use self::{
    igdevice::IGAndroidDevice,
    igrequests::IGRequest,
};
use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Client, Url,
};
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;

static BASE_IG_API_V1: &'static str = "https://i.instagram.com/api/v1/";

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

pub struct IGLoggedOutClient;

impl IGLoggedOutClient {
    pub async fn login(username: &str, password: &str) -> Result<IGLoggedInClient> {
        let cookie_store = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            .build()
            .unwrap();
        let mut ig_client_config = IGClientConfig {
            guid: Uuid::new_v4().to_string(),
            device: IGAndroidDevice::new("1234"),
            csrftoken: "missing".to_string(),
            cookies_str: "".to_string(),
        };
        let qe_sync_response = igrequests::qe_sync::QeRequest {
            metadata: igrequests::IGLoggedOutRequestMetadata::from(&ig_client_config),
            experiments:  igconstants::IG_EXPERIMENTS.to_string(),
        }
        .send(&client, &mut ig_client_config)
        .await?;

        if let igrequests::qe_sync::QeResponse::Fail { .. } = qe_sync_response {
            return Err(IGClientErr::IGLoginError(
                IGLoginErrorResponse::QeSyncResponse(qe_sync_response),
            ));
        }

        let login_response = igrequests::accounts_login::LoginRequest {
            metadata: igrequests::IGLoggedOutRequestMetadata::from(&ig_client_config),
            username: username.to_string(),
            enc_password: format!("#PWD_INSTAGRAM:0:&:{password}"),
            login_attempt_account: 0,
        }
        .send(&client, &mut ig_client_config)
        .await?;

        if let igrequests::accounts_login::LoginResponse::Fail { .. } = login_response {
            return Err(IGClientErr::IGLoginError(
                IGLoginErrorResponse::AccountsLoginResponse(login_response),
            ));
        }

        Ok(IGLoggedInClient {
            client,
            cookie_store,
            ig_client_config,
        })
    }

    pub fn with_ig_client_config(ig_client_config: IGClientConfig) -> IGLoggedInClient {
        let cookie_store = Arc::new(Jar::default());
        cookie_store.add_cookie_str(&ig_client_config.cookies_str, &BASE_IG_API_V1.parse::<Url>().unwrap());
        let client = Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            .build()
            .unwrap();

        IGLoggedInClient {
            client,
            cookie_store,
            ig_client_config: todo!(),
        }
    }
}

pub struct IGLoggedInClient {
    client: Client,
    cookie_store: Arc<Jar>,
    pub ig_client_config: IGClientConfig,
}

impl IGLoggedInClient {
    fn session_cookies(&self) -> Option<String> {
        self.cookie_store
            .cookies(&BASE_IG_API_V1.parse::<Url>().unwrap())
            .map(|hv| hv.to_str().unwrap().to_string())
    }

    pub async fn send<Res>(&mut self, ig_request: &(dyn IGRequest<Res> + Sync)) -> Result<Res>
    where
        Res: DeserializeOwned,
    {
        let response = ig_request
            .send(&self.client, &mut self.ig_client_config)
            .await?;
        
        if let Some(session_cookies) = self.session_cookies() {
            self.ig_client_config.cookies_str = session_cookies;
        }

        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IGClientConfig {
    guid: String,
    device: IGAndroidDevice,
    csrftoken: String,
    cookies_str: String,
}
