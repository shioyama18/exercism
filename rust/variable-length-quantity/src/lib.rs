#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const CONTINUATION_MASK: u8 = 0b1000_0000;
const VALUE_DIGITS: u8 = 0b0111_1111;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter()
        .flat_map(|&number| u32_to_bytes(number))
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut n: u32 = 0;
    let mut output = Vec::new();
    let mut complete_sequence = true;

    for byte in bytes {
        n = n.checked_mul(1 << 7).ok_or(Error::Overflow)?;
        n |= u32::from(byte & VALUE_DIGITS);
        complete_sequence = byte & CONTINUATION_MASK == 0;
        if complete_sequence {
            output.push(n);
            n = 0;
        };
    }

    if complete_sequence {
        Ok(output)
    } else {
        Err(Error::IncompleteNumber)
    }
}

fn u32_to_bytes(number: u32) -> impl Iterator<Item = u8> {
    (0..5)
        .map(move |byte_number| {
            let rotation = (4 - byte_number) * 7;
            let mask = match byte_number {
                4 => 0b0000_0000,
                _ => CONTINUATION_MASK,
            };
            let seven_bit_byte = ((number >> rotation) as u8) & VALUE_DIGITS;
            seven_bit_byte | mask
        })
        .skip_while(|&byte| byte == CONTINUATION_MASK)
}
