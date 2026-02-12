# Production Checklist

## Code Quality
- [ ] No `panic!` in any execution path
- [ ] No `unwrap()` in library code (`src/`)
- [ ] No `unsafe` code
- [ ] All public functions have rustdoc comments
- [ ] All public functions have documented error conditions
- [ ] Errors use the structured `ScriptError` enum
- [ ] All match statements are exhaustive (no catch-all `_ =>` where avoidable)

## Build & Test
- [ ] `cargo build` passes
- [ ] `cargo build --features secp256k1` passes
- [ ] `cargo test` passes (55+ tests)
- [ ] `cargo test --features secp256k1` passes
- [ ] `cargo clippy` has zero warnings
- [ ] `cargo fmt --check` passes
- [ ] `cargo doc --no-deps` builds without warnings

## Documentation
- [ ] Crate-level rustdoc in `lib.rs`
- [ ] Each public module has module-level rustdoc
- [ ] README has usage example
- [ ] README has scope definition (what is / is not implemented)
- [ ] README has security disclaimer
- [ ] README has feature flag documentation

## Packaging
- [ ] `Cargo.toml` has complete metadata (description, license, repo, keywords)
- [ ] Edition is 2021 with `rust-version = "1.63"`
- [ ] Both license files present (MIT + Apache-2.0)
- [ ] `secp256k1` feature flag works correctly

## Examples
- [ ] `cargo run --example p2pkh` compiles and runs
- [ ] `cargo run --example inspect` compiles and runs

## Protocol Correctness
- [ ] Bitcoin truthiness handles negative zero (`0x80`)
- [ ] All four push-data variants work (direct, PUSHDATA1, PUSHDATA2, PUSHDATA4)
- [ ] OP_CHECKSIG stub always pushes true
- [ ] P2PKH uses two-phase execution (not concatenation)
- [ ] Conditional execution (IF/ELSE/ENDIF) handles nesting correctly
- [ ] OP_RETURN immediately fails with `OpReturnEncountered`
- [ ] OP_NOT only flips 0<->1, everything else maps to 0

## Design Integrity
- [ ] Stack is `pub(crate)` — not public API
- [ ] `execute()` is a free function, not a method on a struct
- [ ] `Opcode` is fieldless and derives `Copy`
- [ ] `Token` is separate from `Opcode` (split model)
- [ ] No external `hex` crate dependency (hand-written decode_hex)
- [ ] No `thiserror` dependency (hand-written Display + Error)
- [ ] Execution is deterministic (no randomness, no timestamps, no I/O)
