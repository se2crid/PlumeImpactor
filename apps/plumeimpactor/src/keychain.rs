// #[cfg(all(target_os = "macos", not(debug_assertions)))]
// pub use keyring_core::Entry;
// #[cfg(not(all(target_os = "macos", not(debug_assertions))))]
// pub use keyring::Entry;

const KEYRING_SERVICE: &'static str = "Plume Impactor Credentials";
const KEYRING_USER: &'static str = "Apple ID";
