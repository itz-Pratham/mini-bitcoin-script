use crate::engine::{execute_on_stack, ExecuteOpts};
use crate::error::ScriptError;
use crate::stack::{is_true, Stack};
use crate::tokenizer::parse_script;

/// Validates a Pay-to-Public-Key-Hash (P2PKH) script pair.
///
/// Executes `script_sig` (the unlocking script) on a fresh stack, then
/// executes `script_pubkey` (the locking script) on the resulting stack.
/// This two-phase model matches Bitcoin's actual execution behavior
/// (post-2010), preventing scriptSig from manipulating scriptPubKey's
/// control flow.
///
/// Returns `Ok(true)` if the combined execution succeeds (top stack
/// element is truthy after both phases).
///
/// OP_CHECKSIG uses stub mode (always succeeds). For real ECDSA
/// verification, use [`validate_p2pkh_with_opts`] with a sighash and
/// the `secp256k1` feature enabled.
///
/// Both arguments are raw script bytes (not hex). Use
/// [`crate::hex::decode_hex`] to convert hex strings first.
pub fn validate_p2pkh(script_sig: &[u8], script_pubkey: &[u8]) -> Result<bool, ScriptError> {
    validate_p2pkh_with_opts(script_sig, script_pubkey, &ExecuteOpts::default())
}

/// Validates a P2PKH script pair with execution options.
///
/// See [`validate_p2pkh`] for details. The `opts` parameter controls
/// OP_CHECKSIG behavior via [`ExecuteOpts::sighash`].
pub fn validate_p2pkh_with_opts(
    script_sig: &[u8],
    script_pubkey: &[u8],
    opts: &ExecuteOpts,
) -> Result<bool, ScriptError> {
    let sig_tokens = parse_script(script_sig)?;
    let pk_tokens = parse_script(script_pubkey)?;

    let mut stack = Stack::new();

    // Phase 1: execute scriptSig (pushes sig + pubkey onto stack)
    execute_on_stack(&sig_tokens, &mut stack, opts)?;

    // Phase 2: execute scriptPubKey on the resulting stack
    execute_on_stack(&pk_tokens, &mut stack, opts)?;

    // Final evaluation
    if stack.is_empty() {
        return Ok(false);
    }
    let top = stack.pop()?;
    Ok(is_true(&top))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash;

    /// Builds a scriptSig that pushes a fake signature and a public key.
    fn build_script_sig(sig: &[u8], pubkey: &[u8]) -> Vec<u8> {
        let mut script = Vec::new();
        // Push signature (direct push: length byte + data)
        assert!(sig.len() <= 0x4b);
        script.push(sig.len() as u8);
        script.extend_from_slice(sig);
        // Push public key
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
    fn p2pkh_stub_valid() {
        let fake_sig = b"fake-signature";
        let fake_pubkey = b"fake-public-key-data";
        let pubkey_hash = hash::hash160(fake_pubkey);

        let script_sig = build_script_sig(fake_sig, fake_pubkey);
        let script_pubkey = build_script_pubkey(&pubkey_hash);

        // Stub CHECKSIG always succeeds, so this should pass
        let result = validate_p2pkh(&script_sig, &script_pubkey).unwrap();
        assert!(result);
    }

    #[test]
    fn p2pkh_wrong_pubkey_hash() {
        let fake_sig = b"fake-signature";
        let fake_pubkey = b"fake-public-key-data";
        let wrong_hash = [0xab; 20]; // does not match hash160(fake_pubkey)

        let script_sig = build_script_sig(fake_sig, fake_pubkey);
        let script_pubkey = build_script_pubkey(&wrong_hash);

        // OP_EQUALVERIFY should fail
        let err = validate_p2pkh(&script_sig, &script_pubkey).unwrap_err();
        assert!(matches!(err, ScriptError::VerifyFailed));
    }

    #[test]
    fn p2pkh_empty_scriptsig() {
        let pubkey_hash = [0x00; 20];
        let script_pubkey = build_script_pubkey(&pubkey_hash);

        // Empty scriptSig means stack is empty when scriptPubKey runs,
        // OP_DUP will fail with StackUnderflow
        let err = validate_p2pkh(&[], &script_pubkey).unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow));
    }

    #[test]
    fn p2pkh_with_opts_stub() {
        let fake_sig = b"sig";
        let fake_pubkey = b"key";
        let pubkey_hash = hash::hash160(fake_pubkey);

        let script_sig = build_script_sig(fake_sig, fake_pubkey);
        let script_pubkey = build_script_pubkey(&pubkey_hash);

        let opts = ExecuteOpts { sighash: None };
        let result = validate_p2pkh_with_opts(&script_sig, &script_pubkey, &opts).unwrap();
        assert!(result);
    }

    #[test]
    fn two_phase_isolation() {
        // Verify that scriptSig cannot inject flow control.
        // A scriptSig containing OP_RETURN should fail during phase 1.
        let script_sig = vec![0x6a]; // OP_RETURN
        let script_pubkey = vec![0x51]; // OP_1 (would be true)

        let err = validate_p2pkh(&script_sig, &script_pubkey).unwrap_err();
        assert!(matches!(err, ScriptError::OpReturnEncountered));
    }
}
