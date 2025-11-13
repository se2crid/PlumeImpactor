use serde::{Deserialize};
use serde_json::json;

use super::DeveloperSession;
use crate::SessionRequestTrait;
use crate::auth::account::request::RequestType;
use crate::developer_endpoint;

use crate::Error;

impl DeveloperSession {
    pub async fn v1_list_capabilities(&self, team: &str) -> Result<CapabilitiesResponse, Error> {
        let endpoint = developer_endpoint!("/v1/capabilities");

        let body = json!({ 
            "teamId": team,
            "urlEncodedQueryParams": "filter[platform]=IOS"
        });

        let response = self.v1_send_request(&endpoint, Some(body), Some(RequestType::Get)).await?;
        let response_data: CapabilitiesResponse = serde_json::from_value(response)?;
        
        Ok(response_data)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesResponse {
    pub data: Vec<Capability>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub id: String,
    pub attributes: CapabilityAttributes,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityAttributes {
    pub entitlements: Option<Vec<CapabilityEntitlement>>,
    pub supports_wildcard: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityEntitlement {
    pub profile_key: String,
}
