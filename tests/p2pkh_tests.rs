use mini_bitcoin_script::error::ScriptError;
use mini_bitcoin_script::hash;
use mini_bitcoin_script::script::validate_p2pkh;

/// Builds a scriptSig that pushes a signature and a public key.
fn build_script_sig(sig: &[u8], pubkey: &[u8]) -> Vec<u8> {
    let mut script = Vec::new();
    assert!(sig.len() <= 0x4b);
    script.push(sig.len() as u8);
    script.extend_from_slice(sig);
    assert!(pubkey.len() <= 0x4b);
    script.push(pubkey.len() as u8);
    script.extend_from_slice(pubkey);
    script
}

/// Builds a standard P2PKH scriptPubKey:
/// OP_DUP OP_HASH160 <20-byte-hash> OP_EQUALVERIFY OP_CHECKSIG
fn build_script_pubkey(pubkey_hash: &[u8; 20]) -> Vec<u8> {
    let mut script = Vec::new();
    script.push(0x76); // OP_DUP
    script.push(0xa9); // OP_HASH160
    script.push(0x14); // Push 20 bytes
    script.extend_from_slice(pubkey_hash);
    script.push(0x88); // OP_EQUALVERIFY
    script.push(0xac); // OP_CHECKSIG
    script
}

#[test]
fn p2pkh_valid_stub() {
    // Construct a fake 71-byte signature and 33-byte compressed public key
    let fake_sig = [0x30u8; 71];
    let fake_pubkey = [0x02u8; 33];
    let pubkey_hash = hash::hash160(&fake_pubkey);

    let script_sig = build_script_sig(&fake_sig, &fake_pubkey);
    let script_pubkey = build_script_pubkey(&pubkey_hash);

    let result = validate_p2pkh(&script_sig, &script_pubkey).unwrap();
    assert!(result);
}

#[test]
fn p2pkh_wrong_pubkey_hash() {
    let fake_sig = [0x30u8; 71];
    let fake_pubkey = [0x02u8; 33];
    let wrong_hash = [0xff; 20]; // does not match hash160(fake_pubkey)

    let script_sig = build_script_sig(&fake_sig, &fake_pubkey);
    let script_pubkey = build_script_pubkey(&wrong_hash);

    let err = validate_p2pkh(&script_sig, &script_pubkey).unwrap_err();
    assert_eq!(err, ScriptError::VerifyFailed);
}

#[test]
fn p2pkh_empty_scriptsig() {
    let pubkey_hash = [0x00; 20];
    let script_pubkey = build_script_pubkey(&pubkey_hash);

    let err = validate_p2pkh(&[], &script_pubkey).unwrap_err();
    assert_eq!(err, ScriptError::StackUnderflow);
}

#[test]
fn p2pkh_malformed_script_pubkey() {
    let fake_sig = [0x30u8; 71];
    let fake_pubkey = [0x02u8; 33];

    let script_sig = build_script_sig(&fake_sig, &fake_pubkey);
    // Malformed scriptPubKey: contains an unsupported opcode
    let script_pubkey = vec![0xb0]; // unsupported

    let err = validate_p2pkh(&script_sig, &script_pubkey).unwrap_err();
    assert_eq!(err, ScriptError::UnsupportedOpcode(0xb0));
}
