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