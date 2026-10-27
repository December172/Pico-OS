use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod image;
pub use image::configure_user_image;

const FLASH_ORIGIN: u32 = 0x1000_0000;
const RAM_ORIGIN: u32 = 0x2000_0000;
const RAM_SIZE: u32 = 256 * 1024;

#[derive(Clone, Copy, Debug)]
struct MemoryRegion {
    origin: u32,
    size: u32,
}

impl MemoryRegion {
    fn end(self) -> u32 {
        self.origin + self.size
    }

    fn overlaps(self, other: Self) -> bool {
        self.origin < other.end() && other.origin < self.end()
    }
}

struct BoardLayout {
    name: String,
    flash_end: u32,
    kernel_flash_size: u32,
    kernel_ram_size: u32,
    user_flash: MemoryRegion,
    user_ram: MemoryRegion,
}

#[derive(Clone, Copy, Debug)]
struct ImageLayout {
    flash: MemoryRegion,
    ram: MemoryRegion,
}

fn positive_u32(document: &toml::Value, name: &str) -> u32 {
    let value = nonnegative_u32(document, name);
    assert!(value != 0, "field `{name}` must be a positive u32");
    value
}

fn nonnegative_u32(document: &toml::Value, name: &str) -> u32 {
    document
        .get(name)
        .and_then(toml::Value::as_integer)
        .and_then(|value| u32::try_from(value).ok())
        .unwrap_or_else(|| panic!("field `{name}` must be a nonnegative u32"))
}

fn read_toml(path: &Path) -> toml::Value {
    println!("cargo:rerun-if-changed={}", path.display());
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
        .parse()
        .unwrap_or_else(|error| panic!("invalid TOML in {}: {error}", path.display()))
}

fn read_board_layout(workspace_root: &Path) -> BoardLayout {
    println!("cargo:rerun-if-env-changed=PICO_BOARD");
    let board = env::var("PICO_BOARD").unwrap_or_else(|_| "pico".to_owned());
    assert!(
        !board.is_empty()
            && board
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-'),
        "PICO_BOARD must be a board name, such as pico or pico-16m"
    );
    let board_path = workspace_root.join("boards").join(format!("{board}.toml"));
    println!(
        "cargo:rerun-if-changed={}",
        workspace_root.join("build-support/layout.rs").display()
    );

    let document = read_toml(&board_path);
    let chip = document
        .get("chip")
        .and_then(toml::Value::as_str)
        .expect("board field `chip` must name a supported chip, such as RP2040");
    let target = match chip {
        "RP2040" => "thumbv6m-none-eabi",
        _ => panic!("unsupported board chip `{chip}`; only RP2040 is implemented"),
    };
    assert_eq!(
        env::var("TARGET").expect("Cargo did not provide TARGET"),
        target,
        "{chip} requires {target}"
    );
    println!("cargo:rustc-check-cfg=cfg(board_chip, values(\"RP2040\"))");
    println!("cargo:rustc-cfg=board_chip=\"{chip}\"");

    parse_board_layout(&board, &document)
}

fn parse_board_layout(board: &str, document: &toml::Value) -> BoardLayout {
    let flash_size = positive_u32(document, "flash-size");
    let user_flash_size = positive_u32(document, "user-flash-size");
    let user_ram_size = positive_u32(document, "user-ram-size");

    assert!(
        user_flash_size < flash_size,
        "no space left for kernel flash"
    );
    assert!(user_ram_size < RAM_SIZE, "no space left for kernel RAM");
    for (name, size) in [("user flash", user_flash_size), ("user RAM", user_ram_size)] {
        assert!(
            size >= 256 && size.is_power_of_two(),
            "{name} size must be a power of two and at least 256 bytes"
        );
    }

    let flash_end = FLASH_ORIGIN
        .checked_add(flash_size)
        .expect("board flash address range overflows u32");
    let user_flash_origin = flash_end - user_flash_size;
    let ram_end = RAM_ORIGIN + RAM_SIZE;
    let user_ram_origin = ram_end - user_ram_size;
    assert_eq!(
        user_flash_origin & (user_flash_size - 1),
        0,
        "user flash MPU alignment"
    );
    assert_eq!(
        user_ram_origin & (user_ram_size - 1),
        0,
        "user RAM MPU alignment"
    );
    BoardLayout {
        name: board.to_owned(),
        flash_end,
        kernel_flash_size: flash_size - user_flash_size,
        kernel_ram_size: RAM_SIZE - user_ram_size,
        user_flash: MemoryRegion {
            origin: user_flash_origin,
            size: user_flash_size,
        },
        user_ram: MemoryRegion {
            origin: user_ram_origin,
            size: user_ram_size,
        },
    }
}

fn image_region(
    image: &toml::Value,
    image_name: &str,
    kind: &str,
    pool: MemoryRegion,
    alignment: u32,
) -> MemoryRegion {
    let offset = nonnegative_u32(image, &format!("{kind}-offset"));
    let size = positive_u32(image, &format!("{kind}-size"));
    let end = offset
        .checked_add(size)
        .unwrap_or_else(|| panic!("image `{image_name}` {kind} range overflows u32"));
    assert!(
        end <= pool.size,
        "image `{image_name}` {kind} range exceeds the board user pool"
    );
    assert_eq!(
        offset % alignment,
        0,
        "image `{image_name}` {kind} offset must be aligned to {alignment} bytes"
    );
    // The bounded relative range fits inside the already validated board pool.
    MemoryRegion {
        origin: pool.origin + offset,
        size,
    }
}

fn parse_image_layouts(document: &toml::Value, board: &BoardLayout) -> Vec<(String, ImageLayout)> {
    let images = document
        .get("images")
        .and_then(toml::Value::as_table)
        .expect("images TOML must contain an `images` table");
    let mut layouts: Vec<(String, ImageLayout)> = Vec::new();
    for (name, image) in images {
        let layout = ImageLayout {
            flash: image_region(image, name, "flash", board.user_flash, 2),
            ram: image_region(image, name, "ram", board.user_ram, 8),
        };
        // The current build pipeline embeds the base image at USER_FLASH origin.
        if name == "base" {
            assert_eq!(
                layout.flash.origin, board.user_flash.origin,
                "base image flash-offset must be zero to match its kernel placement"
            );
        }
        for (registered_name, registered) in &layouts {
            assert!(
                !layout.flash.overlaps(registered.flash),
                "images `{name}` and `{registered_name}` overlap in flash"
            );
            assert!(
                !layout.ram.overlaps(registered.ram),
                "images `{name}` and `{registered_name}` overlap in RAM"
            );
        }
        layouts.push((name.clone(), layout));
    }
    layouts
}

/// Generate the selected image's Flash and RAM regions in its own OUT_DIR.
pub fn write_user_memory_script(workspace_root: &Path, image_name: &str) -> PathBuf {
    let board = read_board_layout(workspace_root);
    let document = read_toml(&workspace_root.join("user/images.toml"));
    let image = parse_image_layouts(&document, &board)
        .into_iter()
        .find_map(|(name, layout)| (name == image_name).then_some(layout))
        .unwrap_or_else(|| panic!("missing image configuration: `{image_name}`"));
    let flash_origin = image.flash.origin;
    let flash_size = image.flash.size;
    let ram_origin = image.ram.origin;
    let ram_size = image.ram.size;
    let board_name = &board.name;
    let script = format!(
        r#"/* Generated from boards/{board_name}.toml and user/images.toml ({image_name}). Do not edit. */
MEMORY
{{
    USER_FLASH(rx) : ORIGIN = 0x{flash_origin:08x}, LENGTH = 0x{flash_size:x}
    USER_RAM(rw)   : ORIGIN = 0x{ram_origin:08x}, LENGTH = 0x{ram_size:x}
}}

__user_flash_start = ORIGIN(USER_FLASH);
__user_flash_end = ORIGIN(USER_FLASH) + LENGTH(USER_FLASH);
__user_ram_start = ORIGIN(USER_RAM);
__user_ram_end = ORIGIN(USER_RAM) + LENGTH(USER_RAM);
"#
    );
    write_script(&script)
}

/// Generate the full kernel and user pools in the kernel's own OUT_DIR.
pub fn write_memory_script(workspace_root: &Path) -> PathBuf {
    let layout = read_board_layout(workspace_root);
    let board = &layout.name;
    let flash_end = layout.flash_end;
    let kernel_flash_size = layout.kernel_flash_size;
    let kernel_ram_size = layout.kernel_ram_size;
    let user_flash_origin = layout.user_flash.origin;
    let user_flash_size = layout.user_flash.size;
    let user_ram_origin = layout.user_ram.origin;
    let user_ram_size = layout.user_ram.size;

    let script = format!(
        r#"/* Generated from boards/{board}.toml. Do not edit. */
MEMORY
{{
    FLASH(rx)      : ORIGIN = 0x{FLASH_ORIGIN:08x}, LENGTH = 0x{kernel_flash_size:x}
    USER_FLASH(rx) : ORIGIN = 0x{user_flash_origin:08x}, LENGTH = 0x{user_flash_size:x}
    RAM(rwx)       : ORIGIN = 0x{RAM_ORIGIN:08x}, LENGTH = 0x{kernel_ram_size:x}
    USER_RAM(rw)   : ORIGIN = 0x{user_ram_origin:08x}, LENGTH = 0x{user_ram_size:x}
    SCRATCH_X(rwx) : ORIGIN = 0x20040000, LENGTH = 4K
    SCRATCH_Y(rwx) : ORIGIN = 0x20041000, LENGTH = 4K
}}

__flash_start = 0x{FLASH_ORIGIN:08x};
__flash_end = 0x{flash_end:08x};
__user_flash_start = ORIGIN(USER_FLASH);
__user_flash_end = ORIGIN(USER_FLASH) + LENGTH(USER_FLASH);
__user_ram_start = ORIGIN(USER_RAM);
__user_ram_end = ORIGIN(USER_RAM) + LENGTH(USER_RAM);
"#
    );

    write_script(&script)
}

fn write_script(script: &str) -> PathBuf {
    let out_directory =
        PathBuf::from(env::var_os("OUT_DIR").expect("Cargo did not provide OUT_DIR"));
    let script_path = out_directory.join("memory.ld");
    fs::write(&script_path, script).expect("failed to write memory.ld");
    // INCLUDE memory.ld in the top-level script resolves through this directory.
    println!("cargo:rustc-link-search=native={}", out_directory.display());
    script_path
}
