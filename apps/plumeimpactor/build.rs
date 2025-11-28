use embed_manifest::manifest::{ActiveCodePage, DpiAwareness, HeapType, Setting, SupportedOS::*};
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    // Build scripts are compiled for the host. Detect Windows targets at runtime so
    // cross-compiling (e.g. Linux → Windows) still embeds the manifest.
    let target = std::env::var("TARGET").unwrap_or_default();

    if target.contains("windows") {
        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "plumeimpactor".into());

        // Create a comprehensive manifest for Windows theming and modern features
        let manifest = new_manifest(&pkg_name)
            // Enable modern Windows Common Controls (v6) for theming
            .supported_os(Windows7..=Windows10)
            // Set UTF-8 as active code page for better Unicode support
            .active_code_page(ActiveCodePage::Utf8)
            // Enable heap type optimization for better performance (if available)
            .heap_type(HeapType::SegmentHeap)
            // Enable high-DPI awareness for crisp displays
            .dpi_awareness(DpiAwareness::PerMonitorV2)
            // Enable long path support (if configured in Windows)
            .long_path_aware(Setting::Enabled);

        // Embed the manifest - this works even when cross-compiling!
        if let Err(e) = embed_manifest(manifest) {
            println!("cargo:warning=Failed to embed manifest: {}", e);
            println!("cargo:warning=The application will still work but may lack optimal Windows theming");
        }

        // Keep icon embedding when building on Windows hosts.
        #[cfg(windows)]
        if let Err(e) = compile_icon() {
            println!("cargo:warning=Failed to embed icon: {}", e);
        }

        // Tell Cargo to rerun this build script if the build script changes
        println!("cargo:rerun-if-changed=build.rs");
    }
}

#[cfg(windows)]
fn compile_icon() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = winres::WindowsResource::new();
    res.set_icon("../../package/windows/icon.ico");
    res.compile()?;
    Ok(())
}
