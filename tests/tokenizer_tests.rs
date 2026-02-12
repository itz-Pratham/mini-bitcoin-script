use mini_bitcoin_script::error::ScriptError;
use mini_bitcoin_script::opcode::Opcode;
use mini_bitcoin_script::token::Token;
use mini_bitcoin_script::tokenizer::{parse_script, parse_script_hex};

#[test]
fn empty_script() {
    let tokens = parse_script(&[]).unwrap();
    assert!(tokens.is_empty());
}

#[test]
fn single_opcode() {
    let tokens = parse_script(&[0x76]).unwrap();
    assert_eq!(tokens, vec![Token::Op(Opcode::OpDup)]);
}

#[test]
fn direct_push_3_bytes() {
    let tokens = parse_script(&[0x03, 0xaa, 0xbb, 0xcc]).unwrap();
    assert_eq!(tokens, vec![Token::PushData(vec![0xaa, 0xbb, 0xcc])]);
}

#[test]
fn pushdata1() {
    let tokens = parse_script(&[0x4c, 0x03, 0xaa, 0xbb, 0xcc]).unwrap();
    assert_eq!(tokens, vec![Token::PushData(vec![0xaa, 0xbb, 0xcc])]);
}

#[test]
fn pushdata2() {
    let tokens = parse_script(&[0x4d, 0x03, 0x00, 0xaa, 0xbb, 0xcc]).unwrap();
    assert_eq!(tokens, vec![Token::PushData(vec![0xaa, 0xbb, 0xcc])]);
}

#[test]
fn pushdata4() {
    let tokens = parse_script(&[0x4e, 0x03, 0x00, 0x00, 0x00, 0xaa, 0xbb, 0xcc]).unwrap();
    assert_eq!(tokens, vec![Token::PushData(vec![0xaa, 0xbb, 0xcc])]);
}

#[test]
fn truncated_direct_push() {
    let err = parse_script(&[0x03, 0xaa]).unwrap_err();
    assert_eq!(err, ScriptError::UnexpectedEndOfScript);
}

#[test]
fn op0_parses() {
    let tokens = parse_script(&[0x00]).unwrap();
    assert_eq!(tokens, vec![Token::Op(Opcode::Op0)]);
}

#[test]
fn op1_through_op16() {
    // OP_1 = 0x51, ..., OP_16 = 0x60
    for (i, byte) in (0x51u8..=0x60).enumerate() {
        let tokens = parse_script(&[byte]).unwrap();
        let expected_opcode = match i {
            0 => Opcode::Op1,
            1 => Opcode::Op2,
            2 => Opcode::Op3,
            3 => Opcode::Op4,
            4 => Opcode::Op5,
            5 => Opcode::Op6,
            6 => Opcode::Op7,
            7 => Opcode::Op8,
            8 => Opcode::Op9,
            9 => Opcode::Op10,
            10 => Opcode::Op11,
            11 => Opcode::Op12,
            12 => Opcode::Op13,
            13 => Opcode::Op14,
            14 => Opcode::Op15,
            15 => Opcode::Op16,
            _ => unreachable!(),
        };
        assert_eq!(
            tokens,
            vec![Token::Op(expected_opcode)],
            "failed for byte 0x{byte:02x}"
        );
    }
}

#[test]
fn p2pkh_script_pubkey() {
    // OP_DUP OP_HASH160 <20 bytes> OP_EQUALVERIFY OP_CHECKSIG
    let mut script = vec![0x76, 0xa9, 0x14]; // OP_DUP, OP_HASH160, push 20 bytes
    let hash: [u8; 20] = [0xab; 20];
    script.extend_from_slice(&hash);
    script.push(0x88); // OP_EQUALVERIFY
    script.push(0xac); // OP_CHECKSIG

    let tokens = parse_script(&script).unwrap();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Op(Opcode::OpDup));
    assert_eq!(tokens[1], Token::Op(Opcode::OpHash160));
    assert_eq!(tokens[2], Token::PushData(hash.to_vec()));
    assert_eq!(tokens[3], Token::Op(Opcode::OpEqualVerify));
    assert_eq!(tokens[4], Token::Op(Opcode::OpCheckSig));
}

#[test]
fn unsupported_opcode() {
    let err = parse_script(&[0xb0]).unwrap_err();
    assert_eq!(err, ScriptError::UnsupportedOpcode(0xb0));
}

#[test]
fn parse_script_hex_valid() {
    // OP_1 OP_1 OP_EQUAL -> 0x51 0x51 0x87
    let tokens_hex = parse_script_hex("515187").unwrap();
    let tokens_raw = parse_script(&[0x51, 0x51, 0x87]).unwrap();
    assert_eq!(tokens_hex, tokens_raw);
}

#[test]
fn parse_script_hex_invalid() {
    let err = parse_script_hex("zzzz").unwrap_err();
    assert_eq!(err, ScriptError::InvalidHex);
}

#[test]
fn pushdata1_truncated_length() {
    // OP_PUSHDATA1 with no length byte
    let err = parse_script(&[0x4c]).unwrap_err();
    assert_eq!(err, ScriptError::UnexpectedEndOfScript);
}

#[test]
fn op1negate_parses() {
    let tokens = parse_script(&[0x4f]).unwrap();
    assert_eq!(tokens, vec![Token::Op(Opcode::Op1Negate)]);
}

#[test]
fn multi_token_script() {
    // OP_1 OP_1 OP_EQUAL
    let tokens = parse_script(&[0x51, 0x51, 0x87]).unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Op(Opcode::Op1));
    assert_eq!(tokens[1], Token::Op(Opcode::Op1));
    assert_eq!(tokens[2], Token::Op(Opcode::OpEqual));
}
