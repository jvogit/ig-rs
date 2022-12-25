use crate::igclient::BASE_IG_API_V1;
use serde::{Deserialize, Serialize};

use super::IGPostRequest;

const ACCOUNTS_LOGIN: &'static str = "accounts/login/";

#[derive(Serialize)]
pub struct LoginRequest {
    #[serde(flatten)]
    pub metadata: super::IGRequestMetadata,
    pub username: String,
    pub enc_password: String,
    pub login_attempt_account: u32,
}

impl IGPostRequest<LoginRequest, LoginResponse> for LoginRequest {
    fn url(&self) -> String {
        format!("{BASE_IG_API_V1}{ACCOUNTS_LOGIN}")
    }

    fn payload(&self) -> &LoginRequest {
        self
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
