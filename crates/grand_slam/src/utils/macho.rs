use std::fs;
use std::path::PathBuf;

use apple_codesign::MachFile;
use plist::{Dictionary, Value};

use crate::Error;
    
pub struct MachO<'a> {
    macho_file: MachFile<'a>,
}

impl<'a> MachO<'a> {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path = path.into();
        let macho_data = fs::read(&path)?;
        let macho_data = Box::leak(macho_data.into_boxed_slice());
        let macho_file = MachFile::parse(macho_data)?;

        Ok(MachO {
            macho_file,
        })
    }
    
    pub fn entitlements(&self) -> Result<Option<Dictionary>, Error> {
        let macho = self.macho_file.nth_macho(0)?;
        if let Some(embedded_sig) = macho.code_signature()? {
            if let Ok(Some(slot)) = embedded_sig.entitlements() {
                let value = Value::from_reader_xml(slot.to_string().as_bytes())?;
                if let Value::Dictionary(dict) = value {
                    return Ok(Some(dict));
                }
            }
        }
        Ok(None)
    }
}
