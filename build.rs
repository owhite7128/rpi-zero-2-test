use std::env;
use std::path::Path;

fn main() {
    // Path to the linker script (relative to Cargo.toml)
    let linker_script = "linker.ld"; // Adjust this path if needed

    // Get the absolute path to the linker script
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let script_path = Path::new(&manifest_dir).join(linker_script);

    // Ensure the linker script exists
    if !script_path.exists() {
        panic!(
            "Linker script not found at: {}. Create it or update the path in build.rs.",
            script_path.display()
        );
    }

    // Tell Cargo to re-run the build script if the linker script changes
    println!("cargo:rerun-if-changed={}", script_path.display());

    // Add the linker script argument
    println!("cargo:rustc-link-arg=-T{}", script_path.display());
}