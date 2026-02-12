use crate::error::ScriptError;
use crate::hex::decode_hex;
use crate::opcode::Opcode;
use crate::token::Token;

/// Parses raw script bytes into a sequence of tokens.
///
/// Walks the byte slice left-to-right, dispatching on each byte:
/// - `0x01`-`0x4b`: direct push (byte value = data length)
/// - `0x4c`: OP_PUSHDATA1 (1-byte length prefix)
/// - `0x4d`: OP_PUSHDATA2 (2-byte little-endian length prefix)
/// - `0x4e`: OP_PUSHDATA4 (4-byte little-endian length prefix)
/// - All other bytes: looked up via [`Opcode::from_byte`]
///
/// Returns `ScriptError::UnexpectedEndOfScript` if a push-data instruction
/// extends beyond the end of the byte slice, or
/// `ScriptError::UnsupportedOpcode` for unrecognized byte values.
pub fn parse_script(bytes: &[u8]) -> Result<Vec<Token>, ScriptError> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let len = bytes.len();

    while pos < len {
        let byte = bytes[pos];
        pos += 1;

        match byte {
            // Direct push: byte value is the data length (1-75 bytes)
            0x01..=0x4b => {
                let n = byte as usize;
                if pos + n > len {
                    return Err(ScriptError::UnexpectedEndOfScript);
                }
                tokens.push(Token::PushData(bytes[pos..pos + n].to_vec()));
                pos += n;
            }

            // OP_PUSHDATA1: next 1 byte is the length
            0x4c => {
                if pos >= len {
                    return Err(ScriptError::UnexpectedEndOfScript);
                }
                let n = bytes[pos] as usize;
                pos += 1;
                if pos + n > len {
                    return Err(ScriptError::UnexpectedEndOfScript);
                }
                tokens.push(Token::PushData(bytes[pos..pos + n].to_vec()));
                pos += n;
            }

            // OP_PUSHDATA2: next 2 bytes (little-endian) are the length
            0x4d => {
                if pos + 2 > len {
                    return Err(ScriptError::UnexpectedEndOfScript);
                }
                let n = u16::from_le_bytes([bytes[pos], bytes[pos + 1]]) as usize;
                pos += 2;
                if pos + n > len {
                    return Err(ScriptError::UnexpectedEndOfScript);
                }
                tokens.push(Token::PushData(bytes[pos..pos + n].to_vec()));
                pos += n;
            }

            // OP_PUSHDATA4: next 4 bytes (little-endian) are the length
            0x4e => {
                if pos + 4 > len {
                    return Err(ScriptError::UnexpectedEndOfScript);
                }
                let n = u32::from_le_bytes([
                    bytes[pos],
                    bytes[pos + 1],
                    bytes[pos + 2],
                    bytes[pos + 3],
                ]) as usize;
                pos += 4;
                if pos + n > len {
                    return Err(ScriptError::UnexpectedEndOfScript);
                }
                tokens.push(Token::PushData(bytes[pos..pos + n].to_vec()));
                pos += n;
            }

            // All other bytes: look up as opcode
            _ => match Opcode::from_byte(byte) {
                Some(opcode) => tokens.push(Token::Op(opcode)),
                None => return Err(ScriptError::UnsupportedOpcode(byte)),
            },
        }
    }

    Ok(tokens)
}

/// Parses a hex-encoded script string into tokens.
///
/// Convenience wrapper that decodes the hex string via [`decode_hex`],
/// then passes the resulting bytes to [`parse_script`].
pub fn parse_script_hex(hex: &str) -> Result<Vec<Token>, ScriptError> {
    let bytes = decode_hex(hex)?;
    parse_script(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_script() {
        let tokens = parse_script(&[]).unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn single_opcode() {
        let tokens = parse_script(&[0x76]).unwrap(); // OP_DUP
        assert_eq!(tokens, vec![Token::Op(Opcode::OpDup)]);
    }

    #[test]
    fn direct_push_3_bytes() {
        let tokens = parse_script(&[0x03, 0xaa, 0xbb, 0xcc]).unwrap();
        assert_eq!(tokens, vec![Token::PushData(vec![0xaa, 0xbb, 0xcc])]);
    }

    #[test]
    fn direct_push_truncated() {
        let err = parse_script(&[0x03, 0xaa, 0xbb]).unwrap_err();
        assert!(matches!(err, ScriptError::UnexpectedEndOfScript));
    }

    #[test]
    fn pushdata1() {
        let tokens = parse_script(&[0x4c, 0x02, 0xde, 0xad]).unwrap();
        assert_eq!(tokens, vec![Token::PushData(vec![0xde, 0xad])]);
    }

    #[test]
    fn pushdata1_missing_length() {
        let err = parse_script(&[0x4c]).unwrap_err();
        assert!(matches!(err, ScriptError::UnexpectedEndOfScript));
    }

    #[test]
    fn pushdata1_truncated_data() {
        let err = parse_script(&[0x4c, 0x05, 0x01, 0x02]).unwrap_err();
        assert!(matches!(err, ScriptError::UnexpectedEndOfScript));
    }

    #[test]
    fn pushdata2() {
        // Length = 0x0003 (little-endian: 03 00)
        let tokens = parse_script(&[0x4d, 0x03, 0x00, 0xaa, 0xbb, 0xcc]).unwrap();
        assert_eq!(tokens, vec![Token::PushData(vec![0xaa, 0xbb, 0xcc])]);
    }

    #[test]
    fn pushdata2_missing_length() {
        let err = parse_script(&[0x4d, 0x03]).unwrap_err();
        assert!(matches!(err, ScriptError::UnexpectedEndOfScript));
    }

    #[test]
    fn pushdata4() {
        // Length = 0x00000003 (little-endian: 03 00 00 00)
        let tokens = parse_script(&[0x4e, 0x03, 0x00, 0x00, 0x00, 0xaa, 0xbb, 0xcc]).unwrap();
        assert_eq!(tokens, vec![Token::PushData(vec![0xaa, 0xbb, 0xcc])]);
    }

    #[test]
    fn pushdata4_missing_length() {
        let err = parse_script(&[0x4e, 0x01, 0x00]).unwrap_err();
        assert!(matches!(err, ScriptError::UnexpectedEndOfScript));
    }

    #[test]
    fn unsupported_opcode() {
        let err = parse_script(&[0x50]).unwrap_err(); // OP_RESERVED
        assert!(matches!(err, ScriptError::UnsupportedOpcode(0x50)));
    }

    #[test]
    fn op0_parses() {
        let tokens = parse_script(&[0x00]).unwrap();
        assert_eq!(tokens, vec![Token::Op(Opcode::Op0)]);
    }

    #[test]
    fn multi_token_script() {
        // OP_DUP OP_HASH160 <20 bytes> OP_EQUALVERIFY OP_CHECKSIG
        let mut script = vec![0x76, 0xa9, 0x14]; // OP_DUP, OP_HASH160, push 20 bytes
        script.extend_from_slice(&[0xab; 20]); // 20 bytes of data
        script.push(0x88); // OP_EQUALVERIFY
        script.push(0xac); // OP_CHECKSIG
        let tokens = parse_script(&script).unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Op(Opcode::OpDup));
        assert_eq!(tokens[1], Token::Op(Opcode::OpHash160));
        assert_eq!(tokens[2], Token::PushData(vec![0xab; 20]));
        assert_eq!(tokens[3], Token::Op(Opcode::OpEqualVerify));
        assert_eq!(tokens[4], Token::Op(Opcode::OpCheckSig));
    }

    #[test]
    fn parse_script_hex_roundtrip() {
        let hex = "76a914" // OP_DUP OP_HASH160 push-20
            .to_string()
            + &"ab".repeat(20) // 20 bytes
            + "88ac"; // OP_EQUALVERIFY OP_CHECKSIG
        let tokens = parse_script_hex(&hex).unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::Op(Opcode::OpDup));
        assert_eq!(tokens[4], Token::Op(Opcode::OpCheckSig));
    }

    #[test]
    fn parse_script_hex_invalid() {
        let err = parse_script_hex("zzzz").unwrap_err();
        assert!(matches!(err, ScriptError::InvalidHex));
    }

    #[test]
    fn pushdata1_zero_length() {
        let tokens = parse_script(&[0x4c, 0x00]).unwrap();
        assert_eq!(tokens, vec![Token::PushData(vec![])]);
    }

    #[test]
    fn direct_push_1_byte() {
        let tokens = parse_script(&[0x01, 0xff]).unwrap();
        assert_eq!(tokens, vec![Token::PushData(vec![0xff])]);
    }
}
