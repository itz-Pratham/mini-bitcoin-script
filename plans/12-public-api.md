# 12 — Public API Surface (`lib.rs`)

## File: `src/lib.rs`

## Module Declarations

```rust
pub mod error;
pub mod opcode;
pub mod token;
pub mod tokenizer;
pub mod hash;
pub mod engine;
pub mod script;
pub mod hex;

mod stack;  // pub(crate) — NOT re-exported
```

## Public API Summary

Users of this crate have access to:

### Types
| Type              | Module     | Description                              |
|-------------------|------------|------------------------------------------|
| `ScriptError`     | `error`    | Error enum (9 variants)                  |
| `Opcode`          | `opcode`   | Fieldless opcode enum (27 variants)      |
| `Token`           | `token`    | Parsed script element (Op or PushData)   |
| `ExecuteOpts`     | `engine`   | Execution configuration struct           |

### Functions
| Function               | Module      | Description                         |
|------------------------|-------------|-------------------------------------|
| `parse_script`         | `tokenizer` | Parse raw bytes into tokens         |
| `parse_script_hex`     | `tokenizer` | Parse hex string into tokens        |
| `execute`              | `engine`    | Execute tokens (stub CHECKSIG)      |
| `execute_with_opts`    | `engine`    | Execute with config (sighash)       |
| `validate_p2pkh`       | `script`    | Two-phase P2PKH validation          |
| `validate_p2pkh_with_opts` | `script` | P2PKH with config (sighash)       |
| `decode_hex`           | `hex`       | Hex string to bytes                 |
| `sha256`               | `hash`      | SHA-256 hash                        |
| `ripemd160`            | `hash`      | RIPEMD-160 hash                     |
| `hash160`              | `hash`      | SHA-256 then RIPEMD-160             |
| `hash256`              | `hash`      | Double SHA-256                      |
| `Opcode::from_byte`    | `opcode`    | Byte to opcode conversion           |
| `Opcode::to_byte`      | `opcode`    | Opcode to byte conversion           |

### NOT Public
| Item     | Reason                                            |
|----------|---------------------------------------------------|
| `Stack`  | Internal implementation detail of the engine      |
| `is_true`| Internal truthiness check                         |
| `execute_on_stack` | Internal two-phase execution helper      |

## Crate-Level Documentation

The `lib.rs` doc comment must cover:

1. What Bitcoin Script is (2-3 sentences)
2. What this crate implements (bullet list)
3. What is NOT implemented (exclusion list)
4. OP_CHECKSIG disclaimer (stub by default, real with feature flag)
5. Security disclaimer: "Not consensus-compatible. Do not use for real transactions."
6. Quick usage example (parse P2PKH, execute)
7. Feature flags (document `secp256k1` feature)
