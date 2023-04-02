use crate::igclient::BASE_IG_API_V1;
use serde::{Deserialize, Serialize};

use super::{IGPostRequest, IGRequestMetadata};

const QE_SYNC: &str = "qe/sync/";

#[derive(Serialize, Clone)]
pub struct QeRequest {
    #[serde(default)]
    #[serde(flatten)]
    pub metadata: Option<super::IGRequestMetadata>,
    pub experiments: String,
}

impl QeRequest {
    pub fn new(experiments: String) -> Self {
        Self {
            metadata: None,
            experiments,
        }
    }
}

impl IGPostRequest<QeRequest, QeResponse> for QeRequest {
    fn url(&self) -> String {
        format!("{BASE_IG_API_V1}{QE_SYNC}")
    }

    fn payload(&self, req_metadata: IGRequestMetadata) -> QeRequest {
        Self {
            metadata: Some(req_metadata),
            ..self.clone()
        }
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
