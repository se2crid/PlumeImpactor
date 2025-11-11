mod bundle;
mod device;
mod package;

pub use bundle::Bundle;
pub use bundle::BundleType;
pub use device::Device;
pub use package::Package;

pub trait PlistInfoTrait {
    fn get_name(&self) -> Option<String>;
    fn get_executable(&self) -> Option<String>;
    fn get_bundle_identifier(&self) -> Option<String>;
    fn get_version(&self) -> Option<String>;
    fn get_build_version(&self) -> Option<String>;
}
