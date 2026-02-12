# 10 — Execution Engine (`engine.rs`)

## File: `src/engine.rs`

This is the core module — the largest and most complex. It implements the
stack-based virtual machine that executes parsed Bitcoin Script tokens.

## Public API

```rust
/// Options for script execution.
#[derive(Debug, Clone, Default)]
pub struct ExecuteOpts {
    /// The sighash digest for OP_CHECKSIG verification.
    /// When None, OP_CHECKSIG always pushes true (stub mode).
    /// When Some and the secp256k1 feature is enabled,
    /// real ECDSA signature verification is performed.
    pub sighash: Option<[u8; 32]>,
}

/// Execute a sequence of tokens on a fresh stack.
///
/// Returns Ok(true) if the script succeeds (top stack element is truthy).
/// Returns Ok(false) if stack is empty or top element is falsy.
/// Returns Err(ScriptError) if any operation fails.
///
/// OP_CHECKSIG uses stub mode (always succeeds).
pub fn execute(tokens: &[Token]) -> Result<bool, ScriptError>

/// Execute with configuration options.
pub fn execute_with_opts(
    tokens: &[Token],
    opts: &ExecuteOpts,
) -> Result<bool, ScriptError>
```

## Internal API

```rust
/// Execute tokens on an existing stack. Used by script.rs for two-phase P2PKH.
pub(crate) fn execute_on_stack(
    tokens: &[Token],
    stack: &mut Stack,
    opts: &ExecuteOpts,
) -> Result<(), ScriptError>
```

`execute()` is a thin wrapper:
1. Create fresh `Stack`
2. Call `execute_on_stack(tokens, &mut stack, &ExecuteOpts::default())`
3. Check final stack state

## Conditional Execution Model

The engine maintains a condition stack: `exec_stack: Vec<bool>`.

- When ALL values in `exec_stack` are `true` -> **executing** branch
- When ANY value is `false` -> **skipped** branch

Helper: `fn is_executing(exec_stack: &[bool]) -> bool` — true if exec_stack
is empty (top-level) or all values are true.

### Processing in SKIPPED branch

| Token                  | Action                                            |
|------------------------|---------------------------------------------------|
| `OP_IF` / `OP_NOTIF`  | Push `false` onto exec_stack (nesting balance)    |
| `OP_ELSE`              | If at the depth where we went false, flip top     |
| `OP_ENDIF`             | Pop from exec_stack                               |
| Anything else          | Skip (do nothing)                                 |

### Processing in EXECUTING branch

| Opcode     | Action                                                    |
|------------|-----------------------------------------------------------|
| `OP_IF`    | Pop data stack. Push `is_true(popped)` onto exec_stack    |
| `OP_NOTIF` | Pop data stack. Push `!is_true(popped)` onto exec_stack  |
| `OP_ELSE`  | Flip top of exec_stack                                    |
| `OP_ENDIF` | Pop from exec_stack                                       |

### Error conditions
- `OP_ELSE` with empty exec_stack -> `UnbalancedConditional`
- `OP_ENDIF` with empty exec_stack -> `UnbalancedConditional`
- End of script with non-empty exec_stack -> `UnbalancedConditional`

## Complete Opcode Execution Reference

### PushData
| Token             | Action                         |
|-------------------|--------------------------------|
| `PushData(data)`  | `stack.push(data.clone())`     |

### Constants
| Opcode          | Action                                        |
|-----------------|-----------------------------------------------|
| `Op0`           | `stack.push(vec![])`                          |
| `Op1Negate`     | `stack.push(vec![0x81])`                      |
| `Op1` - `Op16`  | `stack.push(vec![n])` where n = 1..16        |

### Flow Control
| Opcode      | Action                                            |
|-------------|---------------------------------------------------|
| `OpNop`     | No-op                                             |
| `OpIf`      | Conditional model (above)                         |
| `OpNotIf`   | Conditional model (above)                         |
| `OpElse`    | Conditional model (above)                         |
| `OpEndIf`   | Conditional model (above)                         |
| `OpVerify`  | Pop. If `!is_true(val)`, `Err(VerifyFailed)`      |
| `OpReturn`  | `Err(OpReturnEncountered)` immediately            |

### Stack Manipulation
| Opcode      | Before              | After                   |
|-------------|---------------------|-------------------------|
| `OpDup`     | `[..., x]`          | `[..., x, x]`          |
| `OpDrop`    | `[..., x]`          | `[...]`                 |
| `Op2Dup`    | `[..., x1, x2]`     | `[..., x1, x2, x1, x2]`|
| `Op2Drop`   | `[..., x1, x2]`     | `[...]`                 |
| `OpNip`     | `[..., x1, x2]`     | `[..., x2]`            |
| `OpOver`    | `[..., x1, x2]`     | `[..., x1, x2, x1]`   |
| `OpSwap`    | `[..., x1, x2]`     | `[..., x2, x1]`        |
| `OpTuck`    | `[..., x1, x2]`     | `[..., x2, x1, x2]`   |
| `OpDepth`   | `[... (n items)]`    | `[..., n]`             |
| `OpSize`    | `[..., x]`          | `[..., x, len(x)]`    |

For `OpDepth` and `OpSize`: the pushed value is a minimal script integer.
- 0 -> `vec![]` (empty = zero)
- 1-16 -> `vec![n]`
- Larger values would be multi-byte LE, but unlikely in practice

### Comparison
| Opcode          | Action                                              |
|-----------------|-----------------------------------------------------|
| `OpEqual`       | Pop 2. Push `vec![1]` if byte-equal, else `vec![]`  |
| `OpEqualVerify` | OP_EQUAL then OP_VERIFY (compound)                  |

### Logic
| Opcode  | Action                                                  |
|---------|---------------------------------------------------------|
| `OpNot` | Pop. `[0]`/`[]` -> `[1]`. `[1]` -> `[]`. Other -> `[]` |

Bitcoin OP_NOT only flips between 0 and 1. Anything else maps to 0.

### Crypto
| Opcode              | Action                                        |
|---------------------|-----------------------------------------------|
| `OpRipemd160`       | Pop, push `ripemd160(data)` (20 bytes)        |
| `OpSha256`          | Pop, push `sha256(data)` (32 bytes)           |
| `OpHash160`         | Pop, push `hash160(data)` (20 bytes)          |
| `OpHash256`         | Pop, push `hash256(data)` (32 bytes)          |
| `OpCheckSig`        | Pop pubkey, pop sig. See CHECKSIG section.    |
| `OpCheckSigVerify`  | OP_CHECKSIG then OP_VERIFY                    |

## OP_CHECKSIG Behavior

### Default (no feature flag)
1. Pop pubkey (top)
2. Pop signature (second)
3. Push `vec![0x01]` (true) — always succeeds
4. Document: "Stub mode. Signature is not actually verified."

### With `secp256k1` feature + sighash provided
1. Pop pubkey, pop signature
2. If `opts.sighash` is `None`, fall back to stub mode
3. Otherwise:
   a. Strip last byte from signature (hash type byte)
   b. Only support SIGHASH_ALL (`0x01`). If not `0x01`, stub mode.
   c. Parse remaining bytes as DER-encoded ECDSA signature
   d. Parse pubkey as secp256k1 public key
   e. Create Message from sighash
   f. Verify signature
   g. Push `vec![0x01]` if valid, `vec![]` if invalid
   h. If parsing fails, push `vec![]` (false) — matches Bitcoin Core behavior

### Important limitation
Real OP_CHECKSIG requires the full serialized transaction to compute sighash.
This crate does NOT handle transaction serialization. The sighash must be
pre-computed by the caller via `ExecuteOpts`.

## Final Stack Evaluation

After all tokens processed:
1. If `exec_stack` not empty -> `Err(UnbalancedConditional)`
2. If data stack empty -> `Ok(false)`
3. Pop top -> `Ok(is_true(top))`

Note: Bitcoin Core's SCRIPT_VERIFY_CLEANSTACK requires exactly one element.
This is a relay policy, not consensus. We do NOT enforce it.
