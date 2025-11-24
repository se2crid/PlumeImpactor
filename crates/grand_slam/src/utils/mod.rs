mod certificate;
mod provision;
mod macho;

pub use macho::MachO;
pub use provision::MobileProvision;
pub use certificate::CertificateIdentity;

pub fn strip_invalid_name_chars(name: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|', '.'];
    name.chars()
        .filter(|c| !invalid_chars.contains(c))
        .collect()
}
