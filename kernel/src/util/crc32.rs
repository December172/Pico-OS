// see https://www.rfc-editor.org/rfc/rfc1952.html#section-8

const fn make_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut index = 0;

    while index < 256 {
        let mut current = index as u32;
        let mut bit = 0;

        while bit < 8 {
            let last_lsb = current & 0x1;
            current >>= 0x1;
            if last_lsb != 0 {
                current ^= 0xEDB8_8320;
            }
            bit += 1;
        }

        table[index] = current;
        index += 1;
    }

    return table;
}

static CRC32_TABLE: [u32; 256] = make_table();

pub fn update(prev_state: u32, bytes: &[u8]) -> u32 {
    let mut current = !prev_state;

    for &byte in bytes {
        //`0xFF` get lowest byte of current ^ byte
        let index = ((current ^ byte as u32) & 0xFF) as usize;
        current = (current >> 8) ^ CRC32_TABLE[index];
    }

    return !current;
}
