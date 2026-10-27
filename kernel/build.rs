use std::env;
use std::path::PathBuf;

use pico_build_support as layout;

fn main() {
    let manifest_directory = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("Cargo did not provide CARGO_MANIFEST_DIR"),
    );
    let workspace_root = manifest_directory
        .parent()
        .expect("kernel has no parent directory");
    layout::write_memory_script(workspace_root);

    let linker_path = manifest_directory.join("linkers/RP2040.ld");
    println!("cargo:rerun-if-changed={}", linker_path.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_directory
            .join("src/native/boot/rp2040/boot2.bin")
            .display()
    );
    println!(
        "cargo:rustc-link-arg-bin=kernel=-T{}",
        linker_path.display()
    );

    // The base image supplies the idle task and is required for every kernel.
    // Relative paths are resolved from the workspace root, not kernel/.
    println!("cargo:rerun-if-env-changed=PICO_USER_IMAGE");
    let image = env::var_os("PICO_USER_IMAGE")
        .filter(|image| !image.is_empty())
        .expect("PICO_USER_IMAGE is required: build and pack the base image, or run bash tools/build.sh dev");
    let image_path = workspace_root
        .join(image)
        .canonicalize()
        .expect("PICO_USER_IMAGE must name an existing user image object");
    println!("cargo:rerun-if-changed={}", image_path.display());
    println!("cargo:rustc-link-arg-bin=kernel={}", image_path.display());
}
