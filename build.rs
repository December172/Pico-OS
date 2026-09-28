fn main() {
    let linker = if cfg!(feature = "RP2350") {
        "linkers/Pico2.ld"
    } else if cfg!(feature = "Pico_16M") {
        "linkers/Pico_16M.ld"
    } else {
        "linkers/Pico.ld"
    };

    println!("cargo:rustc-link-arg=-T{}", linker);
}