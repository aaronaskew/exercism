#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| {
            let mut value = *value;
            let mut vlq = Vec::new();

            vlq.push((value & 0x7f) as u8);
            value >>= 7;

            while value > 0 {
                vlq.push((value & 0x7f | 0x80) as u8);
                value >>= 7;
            }

            vlq.into_iter().rev()
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut values = Vec::new();
    let mut value = 0u32;

    for (i, byte) in bytes.iter().enumerate() {
        value <<= 7;
        value |= (byte & 0x7f) as u32;

        if byte & 0x80 == 0 {
            values.push(value);
            value = 0;
        } else if i == bytes.len() - 1 {
            return Err(Error::IncompleteNumber);
        }
    }

    Ok(values)
}
