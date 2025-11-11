pub mod auth;
pub mod developer;
pub mod certificate;

use plist::Dictionary;
use serde_json::Value;

use errors::Error;

use crate::auth::account::request::RequestType;

pub use omnisette::AnisetteConfiguration;

trait SessionRequestTrait {
    async fn qh_send_request(&self, endpoint: &str, payload: Option<Dictionary>) -> Result<Dictionary, Error>;
    async fn v1_send_request(&self, url: &str, body: Option<Value>, request_type: Option<RequestType>) -> Result<Value, Error>;
}
