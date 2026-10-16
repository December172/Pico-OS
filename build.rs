#![allow(non_snake_case)]

use std::env;
use std::path::PathBuf;

fn main() {
    let (boardLinker, memoryLinker, architectureLinker, bootAsset, expectedTarget) =
        if cfg!(feature = "Pico_16M") {
            (
                "Pico_16M.ld",
                "memory/Pico_16M.ld",
                "arch/RP2040.ld",
                Some("src/Native/Boot/RP2040/boot2.bin"),
                "thumbv6m-none-eabi",
            )
        } else if cfg!(feature = "RP2040") {
            (
                "Pico.ld",
                "memory/Pico.ld",
                "arch/RP2040.ld",
                Some("src/Native/Boot/RP2040/boot2.bin"),
                "thumbv6m-none-eabi",
            )
        } else {
            panic!("enable an RP2040 board feature before building");
        };

    let target = env::var("TARGET").expect("Cargo must provide TARGET to build.rs");
    if target != expectedTarget {
        panic!("selected board requires target {expectedTarget}, but Cargo is building {target}");
    }

    let manifestDirectory = env::var_os("CARGO_MANIFEST_DIR")
        .expect("Cargo must provide CARGO_MANIFEST_DIR to build.rs");
    let linkerDirectory = PathBuf::from(manifestDirectory).join("linkers");

    for linkerDependency in [boardLinker, memoryLinker, architectureLinker] {
        let linkerPath = linkerDirectory.join(linkerDependency);
        println!("cargo:rerun-if-changed={}", linkerPath.display());
    }

    if let Some(bootAsset) = bootAsset {
        println!("cargo:rerun-if-changed={bootAsset}");
    }

    println!(
        "cargo:rustc-link-search=native={}",
        linkerDirectory.display()
    );
    println!("cargo:rustc-link-arg-bin=kernel=-T{boardLinker}");
}
