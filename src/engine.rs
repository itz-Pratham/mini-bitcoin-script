use crate::error::ScriptError;
use crate::hash;
use crate::opcode::Opcode;
use crate::stack::{is_true, Stack};
use crate::token::Token;

/// Options for script execution.
///
/// Controls optional behavior such as real OP_CHECKSIG verification.
#[derive(Debug, Clone, Default)]
pub struct ExecuteOpts {
    /// The sighash digest for OP_CHECKSIG verification.
    ///
    /// When `None`, OP_CHECKSIG always pushes true (stub mode).
    /// When `Some` and the `secp256k1` feature is enabled,
    /// real ECDSA signature verification is performed.
    pub sighash: Option<[u8; 32]>,
}

/// Executes a sequence of tokens on a fresh stack.
///
/// Returns `Ok(true)` if the script succeeds (top stack element is truthy).
/// Returns `Ok(false)` if the stack is empty or the top element is falsy.
/// Returns `Err(ScriptError)` if any operation fails during execution.
///
/// OP_CHECKSIG uses stub mode (always succeeds). For real signature
/// verification, use [`execute_with_opts`] with a sighash and the
/// `secp256k1` feature enabled.
pub fn execute(tokens: &[Token]) -> Result<bool, ScriptError> {
    execute_with_opts(tokens, &ExecuteOpts::default())
}

/// Executes a sequence of tokens with configuration options.
///
/// See [`execute`] for return value semantics. The `opts` parameter
/// controls OP_CHECKSIG behavior via [`ExecuteOpts::sighash`].
pub fn execute_with_opts(tokens: &[Token], opts: &ExecuteOpts) -> Result<bool, ScriptError> {
    let mut stack = Stack::new();
    execute_on_stack(tokens, &mut stack, opts)?;

    if stack.is_empty() {
        return Ok(false);
    }
    let top = stack.pop()?;
    Ok(is_true(&top))
}

/// Executes tokens on an existing stack.
///
/// Used internally by `script.rs` for two-phase P2PKH execution where
/// the scriptSig runs first, then the scriptPubKey runs on the same stack.
pub(crate) fn execute_on_stack(
    tokens: &[Token],
    stack: &mut Stack,
    opts: &ExecuteOpts,
) -> Result<(), ScriptError> {
    let mut exec_stack: Vec<bool> = Vec::new();

    for token in tokens {
        let executing = is_executing(&exec_stack);

        match token {
            // ── Conditional flow control (always processed) ──────────
            Token::Op(Opcode::OpIf) => {
                if executing {
                    let val = stack.pop()?;
                    exec_stack.push(is_true(&val));
                } else {
                    exec_stack.push(false);
                }
            }
            Token::Op(Opcode::OpNotIf) => {
                if executing {
                    let val = stack.pop()?;
                    exec_stack.push(!is_true(&val));
                } else {
                    exec_stack.push(false);
                }
            }
            Token::Op(Opcode::OpElse) => {
                let top = exec_stack
                    .last_mut()
                    .ok_or(ScriptError::UnbalancedConditional)?;
                *top = !*top;
            }
            Token::Op(Opcode::OpEndIf) => {
                if exec_stack.pop().is_none() {
                    return Err(ScriptError::UnbalancedConditional);
                }
            }

            // ── Skip everything else when not executing ──────────────
            _ if !executing => continue,

            // ── PushData ─────────────────────────────────────────────
            Token::PushData(data) => {
                stack.push(data.clone());
            }

            // ── Constants ────────────────────────────────────────────
            Token::Op(Opcode::Op0) => stack.push(vec![]),
            Token::Op(Opcode::Op1Negate) => stack.push(vec![0x81]),
            Token::Op(Opcode::Op1) => stack.push(vec![1]),
            Token::Op(Opcode::Op2) => stack.push(vec![2]),
            Token::Op(Opcode::Op3) => stack.push(vec![3]),
            Token::Op(Opcode::Op4) => stack.push(vec![4]),
            Token::Op(Opcode::Op5) => stack.push(vec![5]),
            Token::Op(Opcode::Op6) => stack.push(vec![6]),
            Token::Op(Opcode::Op7) => stack.push(vec![7]),
            Token::Op(Opcode::Op8) => stack.push(vec![8]),
            Token::Op(Opcode::Op9) => stack.push(vec![9]),
            Token::Op(Opcode::Op10) => stack.push(vec![10]),
            Token::Op(Opcode::Op11) => stack.push(vec![11]),
            Token::Op(Opcode::Op12) => stack.push(vec![12]),
            Token::Op(Opcode::Op13) => stack.push(vec![13]),
            Token::Op(Opcode::Op14) => stack.push(vec![14]),
            Token::Op(Opcode::Op15) => stack.push(vec![15]),
            Token::Op(Opcode::Op16) => stack.push(vec![16]),

            // ── Flow control ─────────────────────────────────────────
            Token::Op(Opcode::OpNop) => {}
            Token::Op(Opcode::OpVerify) => {
                let val = stack.pop()?;
                if !is_true(&val) {
                    return Err(ScriptError::VerifyFailed);
                }
            }
            Token::Op(Opcode::OpReturn) => {
                return Err(ScriptError::OpReturnEncountered);
            }

            // ── Stack manipulation ───────────────────────────────────
            Token::Op(Opcode::OpDup) => {
                let top = stack.peek()?.to_vec();
                stack.push(top);
            }
            Token::Op(Opcode::OpDrop) => {
                stack.pop()?;
            }
            Token::Op(Opcode::Op2Dup) => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(a.clone());
                stack.push(b.clone());
                stack.push(a);
                stack.push(b);
            }
            Token::Op(Opcode::Op2Drop) => {
                stack.pop()?;
                stack.pop()?;
            }
            Token::Op(Opcode::OpNip) => {
                if stack.len() < 2 {
                    return Err(ScriptError::StackUnderflow);
                }
                stack.remove(stack.len() - 2)?;
            }
            Token::Op(Opcode::OpOver) => {
                if stack.len() < 2 {
                    return Err(ScriptError::StackUnderflow);
                }
                let second = stack.pop()?;
                let first = stack.peek()?.to_vec();
                stack.push(second);
                stack.push(first);
            }
            Token::Op(Opcode::OpSwap) => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(b);
                stack.push(a);
            }
            Token::Op(Opcode::OpTuck) => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push(b.clone());
                stack.push(a);
                stack.push(b);
            }
            Token::Op(Opcode::OpDepth) => {
                let depth = stack.len();
                stack.push(encode_num(depth as i64));
            }
            Token::Op(Opcode::OpSize) => {
                let top = stack.peek()?;
                let size = top.len();
                stack.push(encode_num(size as i64));
            }

            // ── Comparison ───────────────────────────────────────────
            Token::Op(Opcode::OpEqual) => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                stack.push_bool(a == b);
            }
            Token::Op(Opcode::OpEqualVerify) => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                if a != b {
                    return Err(ScriptError::VerifyFailed);
                }
            }

            // ── Logic ────────────────────────────────────────────────
            Token::Op(Opcode::OpNot) => {
                let val = stack.pop()?;
                // OP_NOT: 0 -> 1, 1 -> 0, anything else -> 0
                if val.is_empty() || val == [0x00] {
                    stack.push(vec![0x01]);
                } else {
                    stack.push(vec![]);
                }
            }

            // ── Crypto ───────────────────────────────────────────────
            Token::Op(Opcode::OpRipemd160) => {
                let data = stack.pop()?;
                stack.push(hash::ripemd160(&data).to_vec());
            }
            Token::Op(Opcode::OpSha256) => {
                let data = stack.pop()?;
                stack.push(hash::sha256(&data).to_vec());
            }
            Token::Op(Opcode::OpHash160) => {
                let data = stack.pop()?;
                stack.push(hash::hash160(&data).to_vec());
            }
            Token::Op(Opcode::OpHash256) => {
                let data = stack.pop()?;
                stack.push(hash::hash256(&data).to_vec());
            }
            Token::Op(Opcode::OpCheckSig) => {
                checksig(stack, opts)?;
            }
            Token::Op(Opcode::OpCheckSigVerify) => {
                checksig(stack, opts)?;
                let val = stack.pop()?;
                if !is_true(&val) {
                    return Err(ScriptError::VerifyFailed);
                }
            }
        }
    }

    if !exec_stack.is_empty() {
        return Err(ScriptError::UnbalancedConditional);
    }

    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Returns `true` if the execution stack indicates we are in an executing branch.
fn is_executing(exec_stack: &[bool]) -> bool {
    exec_stack.iter().all(|&v| v)
}

/// Encodes a non-negative integer as a minimal Bitcoin Script number.
fn encode_num(n: i64) -> Vec<u8> {
    if n == 0 {
        return vec![];
    }

    let negative = n < 0;
    let mut abs = if negative { (-n) as u64 } else { n as u64 };
    let mut result = Vec::new();

    while abs > 0 {
        result.push((abs & 0xff) as u8);
        abs >>= 8;
    }

    // If the most significant byte has bit 0x80 set, we need an extra byte
    // for the sign bit.
    if result.last().map_or(false, |&b| b & 0x80 != 0) {
        result.push(if negative { 0x80 } else { 0x00 });
    } else if negative {
        let len = result.len();
        result[len - 1] |= 0x80;
    }

    result
}

/// OP_CHECKSIG implementation.
///
/// Default: stub mode (always pushes true).
/// With `secp256k1` feature + sighash: real ECDSA verification.
fn checksig(stack: &mut Stack, opts: &ExecuteOpts) -> Result<(), ScriptError> {
    let pubkey = stack.pop()?;
    let sig = stack.pop()?;

    #[cfg(feature = "secp256k1")]
    {
        if let Some(sighash) = opts.sighash {
            let result = verify_ecdsa(&sig, &pubkey, &sighash);
            stack.push_bool(result);
            return Ok(());
        }
    }

    // Stub mode: suppress unused warning when feature is off
    let _ = (&pubkey, &sig, &opts);
    stack.push(vec![0x01]);
    Ok(())
}

/// Real ECDSA signature verification using secp256k1.
#[cfg(feature = "secp256k1")]
fn verify_ecdsa(sig_bytes: &[u8], pubkey_bytes: &[u8], sighash: &[u8; 32]) -> bool {
    use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

    // Signature must have at least 1 byte (the hash type byte)
    if sig_bytes.is_empty() {
        return false;
    }

    // Last byte is the hash type. We only support SIGHASH_ALL (0x01).
    let hash_type = sig_bytes[sig_bytes.len() - 1];
    if hash_type != 0x01 {
        // Unsupported hash type — fall back to false
        return false;
    }

    let der_sig = &sig_bytes[..sig_bytes.len() - 1];

    let secp = Secp256k1::verification_only();

    let signature = match Signature::from_der(der_sig) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let public_key = match PublicKey::from_slice(pubkey_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let message = match Message::from_digest(*sighash) {
        msg => msg,
    };

    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::Opcode;
    use crate::token::Token;

    // Helper to build Token::Op
    fn op(o: Opcode) -> Token {
        Token::Op(o)
    }

    fn push(data: &[u8]) -> Token {
        Token::PushData(data.to_vec())
    }

    // ── Basic execution ──────────────────────────────────────────────

    #[test]
    fn empty_script_returns_false() {
        assert_eq!(execute(&[]).unwrap(), false);
    }

    #[test]
    fn op0_is_false() {
        assert_eq!(execute(&[op(Opcode::Op0)]).unwrap(), false);
    }

    #[test]
    fn op1_is_true() {
        assert_eq!(execute(&[op(Opcode::Op1)]).unwrap(), true);
    }

    #[test]
    fn push_data_true() {
        assert_eq!(execute(&[push(&[0x42])]).unwrap(), true);
    }

    #[test]
    fn push_data_empty_is_false() {
        assert_eq!(execute(&[push(&[])]).unwrap(), false);
    }

    // ── Constants ────────────────────────────────────────────────────

    #[test]
    fn op1negate_pushes_0x81() {
        let mut stack = Stack::new();
        execute_on_stack(
            &[op(Opcode::Op1Negate)],
            &mut stack,
            &ExecuteOpts::default(),
        )
        .unwrap();
        assert_eq!(stack.pop().unwrap(), vec![0x81]);
    }

    #[test]
    fn op_n_values() {
        for n in 1u8..=16 {
            let opcode = Opcode::from_byte(0x50 + n).unwrap();
            let mut stack = Stack::new();
            execute_on_stack(&[op(opcode)], &mut stack, &ExecuteOpts::default()).unwrap();
            assert_eq!(stack.pop().unwrap(), vec![n]);
        }
    }

    // ── Flow control ─────────────────────────────────────────────────

    #[test]
    fn op_nop() {
        let tokens = [op(Opcode::Op1), op(Opcode::OpNop)];
        assert_eq!(execute(&tokens).unwrap(), true);
    }

    #[test]
    fn op_verify_true() {
        let tokens = [op(Opcode::Op1), op(Opcode::OpVerify), op(Opcode::Op1)];
        assert_eq!(execute(&tokens).unwrap(), true);
    }

    #[test]
    fn op_verify_false() {
        let tokens = [op(Opcode::Op0), op(Opcode::OpVerify)];
        let err = execute(&tokens).unwrap_err();
        assert!(matches!(err, ScriptError::VerifyFailed));
    }

    #[test]
    fn op_return_error() {
        let tokens = [op(Opcode::Op1), op(Opcode::OpReturn)];
        let err = execute(&tokens).unwrap_err();
        assert!(matches!(err, ScriptError::OpReturnEncountered));
    }

    // ── Conditionals ─────────────────────────────────────────────────

    #[test]
    fn if_true_branch() {
        // OP_1 OP_IF OP_2 OP_ENDIF
        let tokens = [
            op(Opcode::Op1),
            op(Opcode::OpIf),
            op(Opcode::Op2),
            op(Opcode::OpEndIf),
        ];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![2]);
    }

    #[test]
    fn if_false_branch() {
        // OP_0 OP_IF OP_2 OP_ENDIF OP_3
        let tokens = [
            op(Opcode::Op0),
            op(Opcode::OpIf),
            op(Opcode::Op2),
            op(Opcode::OpEndIf),
            op(Opcode::Op3),
        ];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        // OP_2 was skipped, only OP_3 remains
        assert_eq!(stack.pop().unwrap(), vec![3]);
        assert!(stack.is_empty());
    }

    #[test]
    fn if_else_true() {
        // OP_1 OP_IF OP_2 OP_ELSE OP_3 OP_ENDIF
        let tokens = [
            op(Opcode::Op1),
            op(Opcode::OpIf),
            op(Opcode::Op2),
            op(Opcode::OpElse),
            op(Opcode::Op3),
            op(Opcode::OpEndIf),
        ];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![2]);
        assert!(stack.is_empty());
    }

    #[test]
    fn if_else_false() {
        // OP_0 OP_IF OP_2 OP_ELSE OP_3 OP_ENDIF
        let tokens = [
            op(Opcode::Op0),
            op(Opcode::OpIf),
            op(Opcode::Op2),
            op(Opcode::OpElse),
            op(Opcode::Op3),
            op(Opcode::OpEndIf),
        ];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![3]);
        assert!(stack.is_empty());
    }

    #[test]
    fn notif_true_skips() {
        // OP_1 OP_NOTIF OP_2 OP_ENDIF OP_3
        let tokens = [
            op(Opcode::Op1),
            op(Opcode::OpNotIf),
            op(Opcode::Op2),
            op(Opcode::OpEndIf),
            op(Opcode::Op3),
        ];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![3]);
        assert!(stack.is_empty());
    }

    #[test]
    fn unbalanced_if() {
        let tokens = [op(Opcode::Op1), op(Opcode::OpIf)];
        let err = execute(&tokens).unwrap_err();
        assert!(matches!(err, ScriptError::UnbalancedConditional));
    }

    #[test]
    fn unbalanced_else() {
        let tokens = [op(Opcode::OpElse)];
        let err = execute(&tokens).unwrap_err();
        assert!(matches!(err, ScriptError::UnbalancedConditional));
    }

    #[test]
    fn unbalanced_endif() {
        let tokens = [op(Opcode::OpEndIf)];
        let err = execute(&tokens).unwrap_err();
        assert!(matches!(err, ScriptError::UnbalancedConditional));
    }

    // ── Stack manipulation ───────────────────────────────────────────

    #[test]
    fn op_dup() {
        let tokens = [push(&[0xaa]), op(Opcode::OpDup)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![0xaa]);
        assert_eq!(stack.pop().unwrap(), vec![0xaa]);
    }

    #[test]
    fn op_drop() {
        let tokens = [op(Opcode::Op1), op(Opcode::Op2), op(Opcode::OpDrop)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![1]);
    }

    #[test]
    fn op_2dup() {
        let tokens = [op(Opcode::Op1), op(Opcode::Op2), op(Opcode::Op2Dup)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.len(), 4);
        assert_eq!(stack.pop().unwrap(), vec![2]);
        assert_eq!(stack.pop().unwrap(), vec![1]);
        assert_eq!(stack.pop().unwrap(), vec![2]);
        assert_eq!(stack.pop().unwrap(), vec![1]);
    }

    #[test]
    fn op_2drop() {
        let tokens = [
            op(Opcode::Op1),
            op(Opcode::Op2),
            op(Opcode::Op3),
            op(Opcode::Op2Drop),
        ];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![1]);
    }

    #[test]
    fn op_nip() {
        let tokens = [op(Opcode::Op1), op(Opcode::Op2), op(Opcode::OpNip)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop().unwrap(), vec![2]);
    }

    #[test]
    fn op_over() {
        let tokens = [op(Opcode::Op1), op(Opcode::Op2), op(Opcode::OpOver)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop().unwrap(), vec![1]);
        assert_eq!(stack.pop().unwrap(), vec![2]);
        assert_eq!(stack.pop().unwrap(), vec![1]);
    }

    #[test]
    fn op_swap() {
        let tokens = [op(Opcode::Op1), op(Opcode::Op2), op(Opcode::OpSwap)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![1]);
        assert_eq!(stack.pop().unwrap(), vec![2]);
    }

    #[test]
    fn op_tuck() {
        // [1, 2] -> [2, 1, 2]
        let tokens = [op(Opcode::Op1), op(Opcode::Op2), op(Opcode::OpTuck)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop().unwrap(), vec![2]);
        assert_eq!(stack.pop().unwrap(), vec![1]);
        assert_eq!(stack.pop().unwrap(), vec![2]);
    }

    #[test]
    fn op_depth() {
        let tokens = [op(Opcode::Op1), op(Opcode::Op2), op(Opcode::OpDepth)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![2]); // depth was 2
    }

    #[test]
    fn op_depth_empty() {
        let tokens = [op(Opcode::OpDepth)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![]); // depth 0 = empty vec
    }

    #[test]
    fn op_size() {
        let tokens = [push(&[0xaa, 0xbb, 0xcc]), op(Opcode::OpSize)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        assert_eq!(stack.pop().unwrap(), vec![3]); // size = 3
        assert_eq!(stack.pop().unwrap(), vec![0xaa, 0xbb, 0xcc]); // original remains
    }

    // ── Comparison ───────────────────────────────────────────────────

    #[test]
    fn op_equal_true() {
        let tokens = [
            push(&[0x01, 0x02]),
            push(&[0x01, 0x02]),
            op(Opcode::OpEqual),
        ];
        assert_eq!(execute(&tokens).unwrap(), true);
    }

    #[test]
    fn op_equal_false() {
        let tokens = [push(&[0x01]), push(&[0x02]), op(Opcode::OpEqual)];
        assert_eq!(execute(&tokens).unwrap(), false);
    }

    #[test]
    fn op_equalverify_pass() {
        let tokens = [
            push(&[0xaa]),
            push(&[0xaa]),
            op(Opcode::OpEqualVerify),
            op(Opcode::Op1),
        ];
        assert_eq!(execute(&tokens).unwrap(), true);
    }

    #[test]
    fn op_equalverify_fail() {
        let tokens = [push(&[0xaa]), push(&[0xbb]), op(Opcode::OpEqualVerify)];
        let err = execute(&tokens).unwrap_err();
        assert!(matches!(err, ScriptError::VerifyFailed));
    }

    // ── Logic ────────────────────────────────────────────────────────

    #[test]
    fn op_not_zero_becomes_one() {
        let tokens = [op(Opcode::Op0), op(Opcode::OpNot)];
        assert_eq!(execute(&tokens).unwrap(), true);
    }

    #[test]
    fn op_not_one_becomes_zero() {
        let tokens = [op(Opcode::Op1), op(Opcode::OpNot)];
        assert_eq!(execute(&tokens).unwrap(), false);
    }

    #[test]
    fn op_not_other_becomes_zero() {
        let tokens = [op(Opcode::Op2), op(Opcode::OpNot)];
        assert_eq!(execute(&tokens).unwrap(), false);
    }

    // ── Crypto ───────────────────────────────────────────────────────

    #[test]
    fn op_sha256() {
        let tokens = [push(b""), op(Opcode::OpSha256)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        let result = stack.pop().unwrap();
        assert_eq!(result.len(), 32);
        assert_eq!(result, hash::sha256(b"").to_vec());
    }

    #[test]
    fn op_hash160() {
        let tokens = [push(b"test"), op(Opcode::OpHash160)];
        let mut stack = Stack::new();
        execute_on_stack(&tokens, &mut stack, &ExecuteOpts::default()).unwrap();
        let result = stack.pop().unwrap();
        assert_eq!(result.len(), 20);
        assert_eq!(result, hash::hash160(b"test").to_vec());
    }

    // ── OP_CHECKSIG stub ─────────────────────────────────────────────

    #[test]
    fn checksig_stub_always_true() {
        let tokens = [push(&[0x00]), push(&[0x00]), op(Opcode::OpCheckSig)];
        assert_eq!(execute(&tokens).unwrap(), true);
    }

    #[test]
    fn checksigverify_stub() {
        let tokens = [
            push(&[0x00]),
            push(&[0x00]),
            op(Opcode::OpCheckSigVerify),
            op(Opcode::Op1),
        ];
        assert_eq!(execute(&tokens).unwrap(), true);
    }

    // ── encode_num ───────────────────────────────────────────────────

    #[test]
    fn encode_num_zero() {
        assert_eq!(encode_num(0), vec![]);
    }

    #[test]
    fn encode_num_positive() {
        assert_eq!(encode_num(1), vec![0x01]);
        assert_eq!(encode_num(127), vec![0x7f]);
        assert_eq!(encode_num(128), vec![0x80, 0x00]); // needs sign byte
        assert_eq!(encode_num(255), vec![0xff, 0x00]);
        assert_eq!(encode_num(256), vec![0x00, 0x01]);
    }

    #[test]
    fn encode_num_negative() {
        assert_eq!(encode_num(-1), vec![0x81]);
        assert_eq!(encode_num(-127), vec![0xff]);
        assert_eq!(encode_num(-128), vec![0x80, 0x80]);
    }

    // ── Stack underflow ──────────────────────────────────────────────

    #[test]
    fn dup_empty_stack() {
        let err = execute(&[op(Opcode::OpDup)]).unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow));
    }

    #[test]
    fn equal_needs_two() {
        let err = execute(&[op(Opcode::Op1), op(Opcode::OpEqual)]).unwrap_err();
        assert!(matches!(err, ScriptError::StackUnderflow));
    }
}
