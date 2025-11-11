use std::fs;
use std::path::PathBuf;

use plist::{Dictionary, Value};

use errors::Error;

pub struct MobileProvision {
    provision_file: PathBuf,
    provisioning_plist: Value,
}

impl MobileProvision {
    pub fn new<P: Into<PathBuf>>(provision_path: P) -> Result<Self, Error> {
        let path = provision_path.into();
        
        if !path.exists() {
            return Err(Error::ProvisioningEntitlementsUnknown);
        }

        let provisioning_plist = Self::extract_plist_from_provision_file(&path)?;

        Ok(Self {
            provision_file: path.clone(),
            provisioning_plist,
        })
    }
    
    pub fn get_file_path(&self) -> &PathBuf {
        &self.provision_file
    }

    fn extract_plist_from_provision_file(provision_file: &PathBuf) -> Result<Value, Error> {
        let data = fs::read(provision_file)?;
        let start = data.windows(6).position(|w| w == b"<plist").ok_or(Error::ProvisioningEntitlementsUnknown)?;
        let end = data.windows(8).rposition(|w| w == b"</plist>").ok_or(Error::ProvisioningEntitlementsUnknown)? + 8;
        let plist_data = &data[start..end];
        let plist = plist::Value::from_reader_xml(plist_data)?;
        Ok(plist)
    }

    fn extract_entitlements_from_provision_file(&self) -> Result<Value, Error> {
        let plist = self.provisioning_plist.clone();
        let dict = plist
            .as_dictionary()
            .and_then(|d| d.get("Entitlements"))
            .and_then(|v| v.as_dictionary())
            .cloned()
            .ok_or(Error::ProvisioningEntitlementsUnknown)?;
        Ok(Value::Dictionary(dict))
    }
    
    pub fn get_entitlements_as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        let provisioning_entitlements_dictionary = self.extract_entitlements_from_provision_file()?;
        provisioning_entitlements_dictionary.to_writer_xml(&mut buf)?;
        Ok(buf)
    }

    pub fn get_entitlements_dictionary(&self) -> Result<Dictionary, Error> {
        let provisioning_entitlements_dictionary = self.extract_entitlements_from_provision_file()?;
        provisioning_entitlements_dictionary
            .as_dictionary()
            .cloned()
            .ok_or(Error::ProvisioningEntitlementsUnknown)
    }
    
}
impl MobileProvision {
    pub fn get_apple_team_id(&self) -> Option<String> {
        let dict = self.get_entitlements_dictionary().ok()?;
        let team_id_opt = dict.get("application-identifier").and_then(|v| v.as_string());

        let prefix_opt = self.provisioning_plist
            .as_dictionary()
            .and_then(|d| d.get("ApplicationIdentifierPrefix"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_string());

        match (team_id_opt, prefix_opt) {
            (Some(team_id), Some(prefix)) => {
                let mut rest = &team_id[prefix.len()..];
                if rest.starts_with('.') {
                    rest = &rest[1..];
                }
                Some(rest.to_string())
            }
            (Some(team_id), None) => Some(team_id.to_string()),
            _ => None,
        }
    }
}
