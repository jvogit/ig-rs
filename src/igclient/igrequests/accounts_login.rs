use crate::igclient::BASE_IG_API_V1;
use serde::{Deserialize, Serialize};

use super::IGPostRequest;

static ACCOUNTS_LOGIN: &'static str = "accounts/login/";

#[derive(Serialize)]
pub struct LoginRequest {
    #[serde(flatten)]
    pub metadata: super::IGLoggedOutRequestMetadata,
    pub username: String,
    pub enc_password: String,
    pub login_attempt_account: u32,
}

impl IGPostRequest<LoginResponse> for LoginRequest {
    fn url(&self) -> String {
        format!("{BASE_IG_API_V1}{ACCOUNTS_LOGIN}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum LoginResponse {
    #[serde(rename = "ok")]
    Ok { logged_in_user: serde_json::Value },
    #[serde(rename = "fail")]
    Fail(serde_json::Value),
}
