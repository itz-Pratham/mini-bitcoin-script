# 03 — Error Model (`error.rs`)

## File: `src/error.rs`

## ScriptError Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    StackUnderflow,
    UnexpectedEndOfScript,
    InvalidPushData,
    UnsupportedOpcode(u8),
    VerifyFailed,
    ScriptFailed,
    OpReturnEncountered,
    UnbalancedConditional,
    InvalidHex,
}
```

## Variant Descriptions

| Variant                  | When it occurs                                       |
|--------------------------|------------------------------------------------------|
| `StackUnderflow`         | Pop/peek when stack has fewer elements than required  |
| `UnexpectedEndOfScript`  | Byte stream ends mid-instruction (push-data truncated)|
| `InvalidPushData`        | Push-data length exceeds remaining bytes, or malformed|
| `UnsupportedOpcode(u8)`  | Byte is a valid Bitcoin opcode but not implemented here. Carries the raw byte for diagnostics |
| `VerifyFailed`           | OP_VERIFY, OP_EQUALVERIFY, or OP_CHECKSIGVERIFY consumed false |
| `ScriptFailed`           | Execution completed but stack empty or top is false   |
| `OpReturnEncountered`    | OP_RETURN hit — script is provably unspendable        |
| `UnbalancedConditional`  | OP_IF/NOTIF/ELSE/ENDIF not properly balanced          |
| `InvalidHex`             | Hex string has odd length or invalid hex characters   |

## Trait Implementations

### `std::fmt::Display`

Each variant gets a human-readable error message:

- `StackUnderflow` -> "stack underflow: not enough elements on the stack"
- `UnexpectedEndOfScript` -> "unexpected end of script"
- `InvalidPushData` -> "invalid push data encoding"
- `UnsupportedOpcode(b)` -> "unsupported opcode: 0x{b:02x}"
- `VerifyFailed` -> "verify failed: top stack element is false"
- `ScriptFailed` -> "script failed: final stack state is false"
- `OpReturnEncountered` -> "OP_RETURN encountered: script is unspendable"
- `UnbalancedConditional` -> "unbalanced conditional: mismatched IF/ELSE/ENDIF"
- `InvalidHex` -> "invalid hex string"

### `std::error::Error`

Hand-implemented. No `source()` override needed (no inner errors). Just the
default implementation.

## Design Notes

- `UnsupportedOpcode(u8)` — NOT `InvalidOpcode`. All 256 byte values are valid
  in Bitcoin protocol. We simply don't implement all of them.
- `OpReturnEncountered` is distinct from `ScriptFailed`. OP_RETURN is intentional
  (unspendable output marker), not a logic failure.
- `VerifyFailed` covers OP_VERIFY, OP_EQUALVERIFY, OP_CHECKSIGVERIFY — same
  failure mode (top element was false).
- No `String`-based errors. No `Box<dyn Error>`. Concrete enum everywhere.
