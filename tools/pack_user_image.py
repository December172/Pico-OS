#!/usr/bin/env python3
"""Pack user ELFs at their linked Flash addresses into a kernel input object."""

import argparse
from pathlib import Path
import struct
import subprocess
import tempfile
import zlib


HEADER = struct.Struct("<4sHH6I")
SEGMENT = struct.Struct("<5I")
ENTRY = struct.Struct("<3I")
ELF_SECTION = struct.Struct("<10I")
CHECKSUM_OFFSET = 12
IMAGE_FORMAT_VERSION = 1
KERNEL_ABI_VERSION = 1


def validate_elf(elf: bytes) -> None:
    """Accept only a linked, little-endian ARM ELF32 image."""
    if len(elf) < 52 or elf[:7] != b"\x7fELF\x01\x01\x01":
        raise ValueError("expected a little-endian ELF32")
    elf_type, machine = struct.unpack_from("<HH", elf, 16)
    if elf_type != 2 or machine != 40:
        raise ValueError("expected a linked ARM executable ELF")


def validate_image(image: bytes) -> None:
    if len(image) < HEADER.size:
        raise ValueError("image is shorter than UserImageHeader")
    magic, version, abi_version, image_size, _, entries, entry_count, segment, _ = (
        HEADER.unpack_from(image)
    )
    if magic != b"PIOS" or version != IMAGE_FORMAT_VERSION:
        raise ValueError(f"expected a Pico-OS v{IMAGE_FORMAT_VERSION} image")
    if abi_version != KERNEL_ABI_VERSION:
        raise ValueError(f"expected kernel ABI v{KERNEL_ABI_VERSION}, got {abi_version}")
    if image_size != len(image):
        raise ValueError(f"header image_size={image_size}, binary size={len(image)}")
    entry_table_end = entries + entry_count * ENTRY.size
    segment_end = segment + SEGMENT.size
    if entry_count == 0 or entries < HEADER.size or entry_table_end > len(image):
        raise ValueError("invalid task entry table")
    if segment < HEADER.size or segment_end > len(image):
        raise ValueError("invalid RAM segment record")
    if entries < segment_end and segment < entry_table_end:
        raise ValueError("task entry table and RAM segment record overlap")

    for index in range(entry_count):
        offset, _, _ = ENTRY.unpack_from(image, entries + index * ENTRY.size)
        if offset & 1 or offset < HEADER.size or offset + 2 > len(image):
            raise ValueError(f"invalid task entry offset at index {index}")

    offset, address, file_size, memory_size, _ = SEGMENT.unpack_from(image, segment)
    if (
        file_size > memory_size
        or offset + file_size > len(image)
        or address + memory_size > 1 << 32
    ):
        raise ValueError("invalid RAM segment")


def wrap_binary(binary_path: Path, object_path: Path, objcopy: str) -> None:
    subprocess.run(
        [
            objcopy, "-I", "binary", "-O", "elf32-littlearm", "-B", "arm",
            "--rename-section", ".data=.user.image,alloc,load,readonly,data,contents",
            "--strip-all",
            str(binary_path), str(object_path),
        ],
        check=True,
    )


def pack_binary(elf_path: Path, binary_path: Path, objcopy: str) -> tuple[bytearray, int]:
    validate_elf(elf_path.read_bytes())
    subprocess.run([objcopy, "-O", "binary", str(elf_path), str(binary_path)], check=True)
    image = bytearray(binary_path.read_bytes())
    validate_image(image)
    # CRC-32/ISO-HDLC: checksum the declared image with this field zeroed.
    struct.pack_into("<I", image, CHECKSUM_OFFSET, 0)
    checksum = zlib.crc32(image)
    struct.pack_into("<I", image, CHECKSUM_OFFSET, checksum)
    binary_path.write_bytes(image)
    return image, checksum


def pack(elf_path: Path, prefix: Path, objcopy: str) -> None:
    bundle([elf_path], prefix, objcopy, "arm-none-eabi-nm")


def flash_start(elf_path: Path, nm: str) -> int:
    # Release ELFs are stripped. Section names and addresses remain available
    # even when the __user_image_start linker symbol has been removed.
    elf = elf_path.read_bytes()
    validate_elf(elf)
    section_offset = struct.unpack_from("<I", elf, 32)[0]
    section_size, section_count, strings_index = struct.unpack_from("<3H", elf, 46)
    if (
        section_size < ELF_SECTION.size
        or strings_index >= section_count
        or section_offset + section_count * section_size > len(elf)
    ):
        raise ValueError(f"{elf_path}: invalid ELF section table")
    strings = ELF_SECTION.unpack_from(elf, section_offset + strings_index * section_size)
    names_offset, names_size = strings[4:6]
    if names_offset + names_size > len(elf):
        raise ValueError(f"{elf_path}: invalid ELF section names")
    names = elf[names_offset:names_offset + names_size]
    for index in range(section_count):
        section = ELF_SECTION.unpack_from(elf, section_offset + index * section_size)
        if names[section[0]:].split(b"\0", 1)[0] == b".user.header":
            return section[3]

    # Support custom linker layouts that only export the original symbol.
    symbols = subprocess.run(
        [nm, "--defined-only", str(elf_path)], check=True, capture_output=True, text=True
    ).stdout
    for line in symbols.splitlines():
        fields = line.split()
        if len(fields) == 3 and fields[2] == "__user_image_start":
            return int(fields[0], 16)
    raise ValueError(f"{elf_path}: missing __user_image_start")


def bundle(elf_paths: list[Path], prefix: Path, objcopy: str, nm: str) -> None:
    """Keep each image's own header/CRC, padding gaps between linked Flash VMAs."""
    if not elf_paths:
        raise ValueError("at least one source ELF is required")
    elf_paths = [path.resolve() for path in elf_paths]
    prefix = prefix.resolve()
    binary_path = Path(f"{prefix}.bin")
    object_path = Path(f"{prefix}.o")
    if any(path in (binary_path, object_path) for path in elf_paths):
        raise ValueError("output must not overwrite a source ELF")
    prefix.parent.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix=".pico-user-bundle-", dir=prefix.parent) as staging:
        staging = Path(staging)
        images = []
        for index, elf_path in enumerate(elf_paths):
            image, checksum = pack_binary(elf_path, staging / f"image-{index}.bin", objcopy)
            # A single image needs no address lookup or nm dependency.
            address = flash_start(elf_path, nm) if len(elf_paths) > 1 else 0
            images.append((address, image))

        base_address = images[0][0]
        if any(address < base_address for address, _ in images):
            raise ValueError("the first ELF must be the base image at the lowest Flash address")
        combined = bytearray()
        for address, image in sorted(images, key=lambda item: item[0]):
            offset = address - base_address
            if offset < len(combined):
                raise ValueError(f"user images overlap in Flash at 0x{address:08x}")
            combined.extend(b"\xff" * (offset - len(combined)))
            combined.extend(image)

        staged_binary = staging / binary_path.name
        staged_object = staging / object_path.name
        staged_binary.write_bytes(combined)
        wrap_binary(staged_binary, staged_object, objcopy)
        staged_binary.replace(binary_path)
        staged_object.replace(object_path)

    if len(images) == 1:
        print(f"{binary_path}: {len(combined)} bytes, CRC32=0x{checksum:08x}")
    else:
        print(f"{binary_path}: {len(combined)} bytes, {len(images)} independently checksummed images")
    print(object_path)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("elf", type=Path, nargs="+", help="base ELF first, followed by other user ELFs")
    parser.add_argument("--output-prefix", type=Path, required=True)
    parser.add_argument("--objcopy", default="arm-none-eabi-objcopy")
    parser.add_argument("--nm", default="arm-none-eabi-nm", help="fallback for ELFs without a .user.header section")
    args = parser.parse_args()
    try:
        if len(args.elf) == 1:
            pack(args.elf[0], args.output_prefix, args.objcopy)
        else:
            bundle(args.elf, args.output_prefix, args.objcopy, args.nm)
    except (OSError, ValueError, struct.error, subprocess.CalledProcessError) as error:
        parser.exit(1, f"pack_user_image: {error}\n")


if __name__ == "__main__":
    main()
