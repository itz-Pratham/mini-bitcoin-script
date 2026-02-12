# 01 — Architecture

## Module Layout

```
mini-bitcoin-script/
├── Cargo.toml
├── README.md
├── LICENSE                    # MIT (existing)
├── LICENSE-APACHE             # Apache-2.0 (to add)
│
├── src/
│   ├── lib.rs                 # Crate root: re-exports, crate-level rustdoc
│   ├── error.rs               # ScriptError enum
│   ├── opcode.rs              # Opcode enum (fieldless, Copy) + byte constants
│   ├── token.rs               # Token enum: Op(Opcode) | PushData(Vec<u8>)
│   ├── tokenizer.rs           # Script byte parser -> Vec<Token>
│   ├── stack.rs               # Stack wrapper (pub(crate), not public API)
│   ├── hash.rs                # sha256, ripemd160, hash160, hash256
│   ├── engine.rs              # Execution engine: execute(), execute_with_opts()
│   ├── script.rs              # High-level validation: validate_p2pkh()
│   └── hex.rs                 # Hex string decoding utility
│
├── examples/
│   ├── p2pkh.rs               # End-to-end P2PKH validation demo
│   └── inspect.rs             # Parse hex script and print tokens
│
└── tests/
    ├── stack_tests.rs
    ├── tokenizer_tests.rs
    ├── engine_tests.rs
    ├── conditionals_tests.rs
    └── p2pkh_tests.rs
```

## Module Dependency Graph

```
error.rs          (no dependencies — leaf module)
  │
  ├── hex.rs      (depends on: error)
  ├── opcode.rs   (depends on: nothing — leaf module)
  │     │
  │     └── token.rs       (depends on: opcode)
  │           │
  │           └── tokenizer.rs  (depends on: opcode, token, error, hex)
  │
  ├── hash.rs     (depends on: nothing — uses sha2, ripemd crates)
  │
  ├── stack.rs    (depends on: error)
  │
  └── engine.rs   (depends on: token, stack, hash, error)
        │
        └── script.rs  (depends on: tokenizer, engine, error)
```

## Key Architectural Decisions

### Split Opcode and Token

- `Opcode` is a fieldless enum deriving `Copy`. Maps 1:1 to protocol byte values.
- `Token` is `Op(Opcode) | PushData(Vec<u8>)`. Carries runtime data.
- Pattern matching on opcodes is clean (no wildcard for push data).
- Opcode can be used as map key, passed by value, compared cheaply.

### Stack is pub(crate)

Users interact via `parse_script()`, `execute()`, `validate_p2pkh()`. Exposing
the stack would let users construct partial execution states. The stack is an
internal implementation detail.

### Free Function Engine

`execute(tokens)` is simpler than `Engine::new().execute(tokens)`. The engine
has no persistent state between executions. A struct would add ceremony without
benefit. `execute_with_opts()` accepts `ExecuteOpts` for the sighash extension.

### Two-Phase P2PKH

Real Bitcoin (post-2010) executes scriptSig first, then scriptPubKey on the
resulting stack. We follow this model. The concatenation model was deprecated.

### Hand-Implemented Error Traits

~10 error variants, ~30 lines of Display + Error impl. No proc-macro dependency
needed. Keeps the dependency tree minimal.

### Edition 2021

Edition 2024 requires Rust 1.85+ with limited ecosystem adoption. Edition 2021
maximizes compatibility. MSRV: `rust-version = "1.63"`.
