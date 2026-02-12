# 11 — P2PKH Validation (`script.rs`)

## File: `src/script.rs`

## Public API

```rust
/// Validate a Pay-to-Public-Key-Hash (P2PKH) script.
///
/// Executes scriptSig on a fresh stack, then executes scriptPubKey on the
/// resulting stack. This two-phase model matches Bitcoin's actual execution
/// behavior (post-2010).
///
/// OP_CHECKSIG uses stub mode unless sighash is provided via
/// validate_p2pkh_with_opts() and the secp256k1 feature is enabled.
pub fn validate_p2pkh(
    script_sig: &[u8],
    script_pubkey: &[u8],
) -> Result<bool, ScriptError>

/// Validate P2PKH with execution options (e.g., sighash for real CHECKSIG).
pub fn validate_p2pkh_with_opts(
    script_sig: &[u8],
    script_pubkey: &[u8],
    opts: &ExecuteOpts,
) -> Result<bool, ScriptError>
```

## P2PKH Script Pattern

### scriptPubKey (locking script)
```
OP_DUP OP_HASH160 <20-byte-pubkey-hash> OP_EQUALVERIFY OP_CHECKSIG
```

### scriptSig (unlocking script)
```
<signature> <public-key>
```

## Two-Phase Execution Flow

```
validate_p2pkh(script_sig, script_pubkey):
    1. sig_tokens = parse_script(script_sig)?
    2. pk_tokens  = parse_script(script_pubkey)?
    3. stack = Stack::new()
    4. execute_on_stack(&sig_tokens, &mut stack, &opts)?
    5. execute_on_stack(&pk_tokens, &mut stack, &opts)?
    6. if stack.is_empty():
           return Ok(false)
       top = stack.pop()?
       return Ok(is_true(&top))
```

## Step-by-Step P2PKH Trace

Starting with scriptSig: `<sig> <pubkey>`

After phase 1 (scriptSig execution):
```
Stack: [sig, pubkey]  (pubkey on top)
```

Phase 2 (scriptPubKey execution):
```
OP_DUP       -> Stack: [sig, pubkey, pubkey]
OP_HASH160   -> Stack: [sig, pubkey, hash160(pubkey)]
<hash>       -> Stack: [sig, pubkey, hash160(pubkey), expected_hash]
OP_EQUALVERIFY -> Compare top two. If equal, continue. If not, VerifyFailed.
             -> Stack: [sig, pubkey]
OP_CHECKSIG  -> Pop pubkey, pop sig. Verify (or stub). Push result.
             -> Stack: [true]  (if valid)
```

Final: top is `[0x01]` = true -> `Ok(true)`

## Why Two-Phase (Not Concatenation)

The concatenation model (`scriptSig + scriptPubKey` as one script) was used
before 2010. It was replaced because it allowed scriptSig to directly manipulate
the scriptPubKey's control flow. The two-phase model prevents this by:

1. Running scriptSig first (can only push data onto stack)
2. Running scriptPubKey on the resulting stack (has the logic)

This is the model Bitcoin Core uses today. Our implementation matches it.

## Error Propagation

Any error during phase 1 or phase 2 propagates up immediately:
- Parse error in scriptSig -> `ScriptError` from tokenizer
- Parse error in scriptPubKey -> `ScriptError` from tokenizer
- Execution error in either phase -> `ScriptError` from engine
- Wrong pubkey hash -> `VerifyFailed` (from OP_EQUALVERIFY)
