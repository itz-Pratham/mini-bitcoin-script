use crate::error::ScriptError;

/// Decode a hexadecimal string into a byte vector.
///
/// Accepts both uppercase and lowercase hex digits. Does not accept
/// a `0x` prefix — callers must strip it if present.
///
/// # Errors
///
/// Returns [`ScriptError::InvalidHex`] if the string has an odd number
/// of characters or contains non-hex characters.
pub fn decode_hex(hex: &str) -> Result<Vec<u8>, ScriptError> {
    if hex.len() % 2 != 0 {
        return Err(ScriptError::InvalidHex);
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);

    for i in (0..hex.len()).step_by(2) {
        let pair = &hex[i..i + 2];
        let byte = u8::from_str_radix(pair, 16).map_err(|_| ScriptError::InvalidHex)?;
        bytes.push(byte);
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(decode_hex("").unwrap(), vec![]);
    }

    #[test]
    fn single_byte() {
        assert_eq!(decode_hex("00").unwrap(), vec![0x00]);
        assert_eq!(decode_hex("ff").unwrap(), vec![0xff]);
    }

    #[test]
    fn mixed_case() {
        assert_eq!(decode_hex("FF").unwrap(), vec![0xff]);
        assert_eq!(decode_hex("aAbB").unwrap(), vec![0xaa, 0xbb]);
    }

    #[test]
    fn multi_byte() {
        assert_eq!(
            decode_hex("deadbeef").unwrap(),
            vec![0xde, 0xad, 0xbe, 0xef]
        );
    }

    #[test]
    fn odd_length() {
        assert_eq!(decode_hex("0"), Err(ScriptError::InvalidHex));
        assert_eq!(decode_hex("abc"), Err(ScriptError::InvalidHex));
    }

    #[test]
    fn invalid_characters() {
        assert_eq!(decode_hex("gg"), Err(ScriptError::InvalidHex));
        assert_eq!(decode_hex("0x00"), Err(ScriptError::InvalidHex));
    }
}
