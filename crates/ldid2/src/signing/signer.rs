use std::fs;
use std::path::PathBuf;

use apple_codesign::{SigningSettings, UnifiedSigner};

use errors::Error;
use crate::{certificate::Certificate, provision::MobileProvision};
use super::signer_settings::{SignerSettings, SignerMode};
use types::{Bundle, PlistInfoTrait};

pub struct Signer {
    certificate: Option<Certificate>,
    settings: SignerSettings,
    provisioning_files: Vec<MobileProvision>,
}

impl Signer {
    pub fn new(
        certificate: Option<Certificate>,
        settings: SignerSettings,
        provisioning_files: Vec<MobileProvision>,
    ) -> Self {
        Self {
            certificate,
            settings,
            provisioning_files,
        }
    }

    pub fn sign(&self, path: PathBuf) -> Result<(), Error> {
        let bundle = Bundle::new(path.clone())?;
        let bundles = bundle.get_embedded_bundles()?;

        if let Some(new_identifier) = self.settings.custom_identifier.as_ref() {
            if let Some(old_identifier) = bundle.get_bundle_identifier() {
                for embedded_bundle in &bundles {
                    embedded_bundle.set_matching_identifier(
                        &old_identifier,
                        &new_identifier,
                    )?;
                }
                
                bundle.set_bundle_identifier(new_identifier)?;
            }
        }
        
        if let Some(new_name) = self.settings.custom_name.as_ref() {
            bundle.set_name(new_name)?;
        }
        
        if let Some(new_version) = self.settings.custom_build_version.as_ref() {
            bundle.set_version(new_version)?;
        }

        match self.settings.sign_mode {
            SignerMode::Zsign => {
                if let Some(prov) = self.provisioning_files.get(0) {
                    if self.settings.embed_mobileprovision {
                        fs::copy(prov.get_file_path(), bundle.get_dir().join("embedded.mobileprovision"))?;
                    }

                    let mut settings = self.build_base_settings(false)?;
                    if let Ok(ent_xml) = prov.get_entitlements_as_bytes() {
                    settings
                        .set_entitlements_xml(apple_codesign::SettingsScope::Main, String::from_utf8_lossy(&ent_xml))
                        .ok();
                    }
                    
                    UnifiedSigner::new(settings).sign_path_in_place(bundle.get_dir())?;
                }
            }
            SignerMode::Default => {
                let mut sorted_bundles = bundles.clone();
                sorted_bundles.push(bundle.clone());
                sorted_bundles.sort_by_key(|b| b.get_dir().components().count());
                sorted_bundles.reverse();

                for bundle in &sorted_bundles {
                    
                    let mut settings = self.build_base_settings(true)?;

                    if bundle._type == types::BundleType::AppExtension || bundle._type == types::BundleType::App {
                        let mut matched_prov = None;
                        
                        println!("hii");
                        println!("bundleid: {}", bundle.get_bundle_identifier().unwrap_or("no bundle id".to_string()));

                        for prov in &self.provisioning_files {
                            println!("Checking provision: {:?}", prov.get_file_path());
                            println!("teamid: {}", prov.get_apple_team_id().unwrap_or("no team id".to_string()));
                            if let (Some(bundle_id), Some(team_id)) = (bundle.get_bundle_identifier(), prov.get_apple_team_id()) {
                                if team_id == bundle_id {
                                    matched_prov = Some(prov);
                                    break;
                                }
                            }
                        }

                        let prov = matched_prov.unwrap_or_else(|| &self.provisioning_files[0]);
                        fs::copy(prov.get_file_path(), bundle.get_dir().join("embedded.mobileprovision"))?;
                        println!("Moved {:?} to {:?}", prov.get_file_path(), bundle.get_dir().join("embedded.mobileprovision"));

                        if let Ok(ent_xml) = prov.get_entitlements_as_bytes() {
                            settings
                                .set_entitlements_xml(apple_codesign::SettingsScope::Main, String::from_utf8_lossy(&ent_xml))
                                .ok();
                        }
                    }

                    UnifiedSigner::new(settings).sign_path_in_place(bundle.get_dir())?;
                }
            }
        }

        if let Some(cert) = &self.certificate {
            if let Some(key) = &cert.key {
                key.finish()?;
            }
        }

        Ok(())
    }

    fn build_base_settings(&self, shallow_override: bool) -> Result<SigningSettings<'_>, Error> {
        let mut settings = SigningSettings::default();
        if let Some(cert) = &self.certificate {
            cert.load_into_signing_settings(&mut settings)?;
            settings.set_team_id_from_signing_certificate();
        }
        settings.set_for_notarization(false);
        settings.set_shallow(shallow_override || self.settings.sign_shallow);
        Ok(settings)
    }
}
