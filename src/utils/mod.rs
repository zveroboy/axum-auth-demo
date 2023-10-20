use std::collections::HashMap;

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

pub fn hex(input: &[u8]) -> String {
    input
        .iter()
        .flat_map(|b| {
            [
                HEX_CHARS[(b >> 4) as usize] as char,
                HEX_CHARS[(b & 0x0f) as usize] as char,
            ]
        })
        .collect::<String>()
}

pub fn hex_literal(input: &str) -> Option<Vec<u8>> {
    let hex_len = input.len();
    if hex_len % 2 != 0 {
        return None;
    }

    let mut i = 0;
    let mut bytes = Vec::with_capacity(hex_len / 2);
    while i + 1 < hex_len {
        let byte_chunk = &input[i..=i + 1];
        let byte = u8::from_str_radix(byte_chunk, 16).unwrap();
        bytes.push(byte);
        i += 2;
    }
    Some(bytes)
}
