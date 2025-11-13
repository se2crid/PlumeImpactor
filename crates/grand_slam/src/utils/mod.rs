mod certificate;
mod provision;
mod macho;
mod signer;
mod bundle;

pub use macho::MachO;
pub use provision::MobileProvision;
pub use certificate::Certificate;
pub use signer::Signer;
pub use bundle::Bundle;
pub use bundle::BundleType;

pub struct SignerSettings {
    pub should_embed_provisioning: bool,
    pub custom_name: Option<String>,
    pub custom_identifier: Option<String>,
    pub custom_build_version: Option<String>,
    pub support_minimum_os_version: Option<bool>,
    pub support_file_sharing: Option<bool>,
    pub support_pro_motion: Option<bool>,
    pub support_ipad_fullscreen: Option<bool>,
    pub remove_url_schemes: Option<bool>,
}

impl Default for SignerSettings {
    fn default() -> Self {
        Self {
            should_embed_provisioning: true,
            custom_name: None,
            custom_identifier: None,
            custom_build_version: None,
            support_minimum_os_version: None,
            support_file_sharing: None,
            support_pro_motion: None,
            support_ipad_fullscreen: None,
            remove_url_schemes: None,
        }
    }
}

pub trait PlistInfoTrait {
    fn get_name(&self) -> Option<String>;
    fn get_executable(&self) -> Option<String>;
    fn get_bundle_identifier(&self) -> Option<String>;
    fn get_version(&self) -> Option<String>;
    fn get_build_version(&self) -> Option<String>;
}
