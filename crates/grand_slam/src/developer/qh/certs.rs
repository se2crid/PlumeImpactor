use serde::Deserialize;
use plist::{Data, Date, Dictionary, Integer, Value};

use crate::Error;

use crate::{SessionRequestTrait, developer_endpoint};
use super::{DeveloperSession, ResponseMeta};

impl DeveloperSession {
    pub async fn qh_list_certs(&self, team_id: &str) -> Result<CertsResponse, Error> {
        let endpoint = developer_endpoint!("/QH65B2/ios/listAllDevelopmentCerts.action");
        
        let mut body = Dictionary::new();
        body.insert("teamId".to_string(), Value::String(team_id.to_string()));
        
        let response = self.qh_send_request(&endpoint, Some(body)).await?;
        let response_data: CertsResponse = plist::from_value(&Value::Dictionary(response))?;

        Ok(response_data)
    }
    
    pub async fn qh_revoke_cert(&self, team_id: &str, serial_number: &str) -> Result<ResponseMeta, Error> {
        let endpoint = developer_endpoint!("/QH65B2/ios/revokeDevelopmentCert.action");
        
        let mut body = Dictionary::new();
        body.insert("teamId".to_string(), Value::String(team_id.to_string()));
        body.insert("serialNumber".to_string(), Value::String(serial_number.to_string()));
        
        let response = self.qh_send_request(&endpoint, Some(body)).await?;
        let response_data: ResponseMeta = plist::from_value(&Value::Dictionary(response))?;

        Ok(response_data)
    }

    // pub async fn qh_submit_cert_csr(&self, team_id: &str, csr_data: &[u8], machine_name: &str) -> Result<Cert, Error> {
    //     let endpoint = developer_endpoint!("/QH65B2/ios/submitDevelopmentCSR.action");
        
    //     let mut body = Dictionary::new();
    //     body.insert("teamId".to_string(), Value::String(team_id.to_string()));
    //     body.insert("csrContent".to_string(), Value::Data(Data::from(csr_data.to_vec())));
    //     body.insert("machineId".to_string(), Value::String(Uuid::new_v4().to_string().to_uppercase()));
    //     body.insert("machineName".to_string(), Value::String(machine_name.to_string()));
        
    //     let response = self.send_request(&endpoint, Some(body)).await?;
    //     println!("{:#?}", response);
    //     // let response_data: Cert = plist::from_value(&Value::Dictionary(response))?;

    //     // Ok(response_data)
    //     todo!("Implement CSR submission")
    // }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CertsResponse {
    pub certificates: Vec<Cert>,
    #[serde(flatten)]
    pub meta: ResponseMeta,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cert {
    pub name: String,
    pub certificate_id: String,
    pub serial_number: String,
    pub status: String,
    status_code: Integer,
    pub expiration_date: Date,
    certificate_platform: Option<String>,
    pub cert_type: Option<CertType>,
    pub cert_content: Data,
    machine_id: Option<String>,
    machine_name: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CertType {
    certificate_type_display_id: String,
    pub name: String,
    platform: String,
    permission_type: String,
    distribution_method: String,
    owner_type: String,
    days_overlap: Integer,
    max_active: Integer,
}
