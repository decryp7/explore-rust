use std::env;
use copy_to_output::copy_to_output;

fn main() {
    // Re-runs script if any files in res are changed
    println!("cargo:rerun-if-changed=lib/*");

    #[cfg(target_os = "macos")]
    copy_to_output("lib/libpdfium.dylib", &env::var("PROFILE").unwrap()).expect("Could not copy");

    #[cfg(target_os = "windows")]
    copy_to_output("lib/pdfium.dll", &env::var("PROFILE").unwrap()).expect("Could not copy");

    #[cfg(target_os = "linux")]
    copy_to_output("lib/libpdfium.so", &env::var("PROFILE").unwrap()).expect("Could not copy");
}