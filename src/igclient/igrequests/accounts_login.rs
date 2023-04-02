use crate::igclient::BASE_IG_API_V1;
use serde::{Deserialize, Serialize};

use super::{IGPostRequest, IGRequestMetadata};

const ACCOUNTS_LOGIN: &'static str = "accounts/login/";

#[derive(Serialize, Clone)]
pub struct LoginRequest {
    #[serde(default)]
    #[serde(flatten)]
    metadata: Option<super::IGRequestMetadata>,
    pub username: String,
    pub enc_password: String,
    pub login_attempt_account: u32,
}

impl LoginRequest {
    pub fn new(username: String, enc_password: String, login_attempt_account: u32) -> Self {
        Self {
            metadata: None,
            username,
            enc_password,
            login_attempt_account,
        }
    }
}

impl IGPostRequest<LoginRequest, LoginResponse> for LoginRequest {
    fn url(&self) -> String {
        format!("{BASE_IG_API_V1}{ACCOUNTS_LOGIN}")
    }

    fn payload(&self, req_metadata: IGRequestMetadata) -> LoginRequest {
        Self {
            metadata: Some(req_metadata),
            ..self.clone()
        }
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
