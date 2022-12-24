use crate::igclient::BASE_IG_API_V1;
use serde::{Deserialize, Serialize};

use super::IGPostRequest;

static QE_SYNC: &'static str = "qe/sync/";

#[derive(Serialize)]
pub struct QeRequest {
    #[serde(flatten)]
    pub metadata: super::IGLoggedOutRequestMetadata,
    pub experiments: String,
}
impl IGPostRequest<QeResponse> for QeRequest {
    fn url(&self) -> String {
        format!("{BASE_IG_API_V1}{QE_SYNC}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum QeResponse {
    #[serde(rename = "ok")]
    Ok(serde_json::Value),
    #[serde(rename = "fail")]
    Fail(serde_json::Value),
}
