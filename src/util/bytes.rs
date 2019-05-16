pub fn bytes_from_u32(value: u32) -> Vec<u8> {
    let mut value_bytes = [0u8; 4];
    value_bytes[0] = value as u8;
    value_bytes[1] = (value >> 8) as u8;
    value_bytes[2] = (value >> 16) as u8;
    value_bytes[3] = (value >> 24) as u8;

    value_bytes.to_vec()
}

// Converts the first 4 bytes of a Vec<u8> to a u32 using Little Endian encoding.
// Must include at least 4 bytes or default 0 is returned.
pub fn bytes_to_u32(value: Vec<u8>) -> u32 {
    let mut num = 0;
    if value.len() >= 4 {
        num = (value[0] as u32)
            + ((value[1] as u32) << 8)
            + ((value[2] as u32) << 16)
            + ((value[3] as u32) << 24);
    }

    num
}

pub fn bytes_from_u64(value: u64) -> Vec<u8> {
    let mut value_bytes = [0u8; 8];
    value_bytes[0] = value as u8;
    value_bytes[1] = (value >> 8) as u8;
    value_bytes[2] = (value >> 16) as u8;
    value_bytes[3] = (value >> 24) as u8;
    value_bytes[4] = (value >> 32) as u8;
    value_bytes[5] = (value >> 40) as u8;
    value_bytes[6] = (value >> 48) as u8;
    value_bytes[7] = (value >> 56) as u8;

    value_bytes.to_vec()
}

// Converts the first 8 bytes of a Vec<u8> to a u64 using Little Endian encoding.
// Must include at least 8 bytes or default 0 is returned.
pub fn bytes_to_u64(value: Vec<u8>) -> u64 {
    let mut num = 0;
    if value.len() >= 8 {
        num = (value[0] as u64)
            + ((value[1] as u64) << 8)
            + ((value[2] as u64) << 16)
            + ((value[3] as u64) << 24)
            + ((value[4] as u64) << 32)
            + ((value[5] as u64) << 40)
            + ((value[6] as u64) << 48)
            + ((value[7] as u64) << 56);
    }

    num
}
