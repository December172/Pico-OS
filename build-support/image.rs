use std::env;
use std::fs;
use std::mem::{offset_of, size_of};
use std::path::PathBuf;

use pico_abi::image::*;

/// Generate linker inputs shared by independently linked user images.
pub fn configure_user_image(image_name: &str, binary_name: &str) {
    let manifest_directory = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("Cargo did not provide CARGO_MANIFEST_DIR"),
    );
    let workspace_root = manifest_directory
        .parent()
        .and_then(|parent| parent.parent())
        .expect("user image is not under user/");
    crate::write_user_memory_script(workspace_root, image_name);

    for path in [
        workspace_root.join("build-support/image.rs"),
        manifest_directory.join("Cargo.toml"),
        manifest_directory.join("linker.ld"),
        manifest_directory.join("task-entries.ld"),
        workspace_root.join("user/linkers/user-sections.ld"),
    ] {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    for directory in [
        workspace_root.join("user/linkers"),
        manifest_directory.clone(),
    ] {
        println!("cargo:rustc-link-search=native={}", directory.display());
    }

    let symbols = [
        ("__image_magic", u32::from_le_bytes(IMAGE_MAGIC)),
        ("__image_format_version", u32::from(IMAGE_FORMAT_VERSION)),
        ("__kernel_abi_version", u32::from(KERNEL_ABI_VERSION)),
        ("__image_header_size", size_of::<UserImageHeader>() as u32),
        (
            "__image_checksum_offset",
            offset_of!(UserImageHeader, crc32) as u32,
        ),
        ("__task_entry_size", size_of::<UserTaskEntry>() as u32),
        ("__image_segment_size", size_of::<UserImageSegment>() as u32),
        ("__user_ram_flags", IMAGE_SEGMENT_READ | IMAGE_SEGMENT_WRITE),
        ("__idle_task_flags", USER_TASK_FLAG_IDLE),
    ];
    let definitions: String = symbols
        .into_iter()
        .map(|(name, value)| format!("{name} = 0x{value:x};\n"))
        .collect();
    let out_directory =
        PathBuf::from(env::var_os("OUT_DIR").expect("Cargo did not provide OUT_DIR"));
    fs::write(
        out_directory.join("image-abi.ld"),
        format!("/* Generated from pico-abi by build.rs. Do not edit. */\n{definitions}"),
    )
    .expect("failed to write image-abi.ld");
    println!(
        "cargo:rustc-link-arg-bin={binary_name}=-T{}",
        manifest_directory.join("linker.ld").display()
    );
}
