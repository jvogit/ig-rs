use crate::igclient::BASE_IG_API_V1;
use serde::{Deserialize, Serialize};

use super::IGGetRequest;

const DIRECT_V2_INBOX: &'static str = "direct_v2/inbox/";

#[derive(Serialize)]
pub struct DirectV2InboxRequest {}

impl IGGetRequest<DirectV2InboxRequest, DirectV2InboxResponse> for DirectV2InboxRequest {
    fn url(&self) -> String {
        format!("{BASE_IG_API_V1}{DIRECT_V2_INBOX}")
    }

    fn query_strings(&self) -> &DirectV2InboxRequest {
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum DirectV2InboxResponse {
    #[serde(rename = "ok")]
    Ok(serde_json::Value),
    #[serde(rename = "fail")]
    Fail(serde_json::Value),
}
