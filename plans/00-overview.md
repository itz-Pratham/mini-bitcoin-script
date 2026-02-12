# 00 — Project Overview

## Project

**mini-bitcoin-script** — Minimal, protocol-accurate Bitcoin Script parsing and
execution engine in Rust.

## Goals

1. Demonstrate deep understanding of Bitcoin Script at the protocol level
2. Showcase systems-level Rust engineering (zero unsafe, zero panics, structured errors)
3. Be publishable on crates.io as a real library crate
4. Serve as high-quality proof-of-work for a Bitcoin-focused engineering program
5. Remain minimal but disciplined — intentional inclusions, explicit exclusions

## What This Crate Implements

- Script byte parsing (tokenizer) with all four push-data encoding variants
- 27 opcodes: constants, flow control, stack manipulation, comparison, logic, crypto
- Conditional execution (OP_IF / OP_NOTIF / OP_ELSE / OP_ENDIF)
- Stack-based execution engine with Bitcoin-accurate truthiness semantics
- Four cryptographic hash functions: SHA-256, RIPEMD-160, HASH160, HASH256
- OP_CHECKSIG as stub by default, optional real ECDSA via `secp256k1` feature flag
- P2PKH validation using two-phase execution (post-2010 protocol-accurate model)
- Structured error types with hand-implemented Display + Error
- Hex decoding utility for working with script hex strings
- Display formatting for opcodes and tokens

## What This Crate Does NOT Implement

- Full consensus rule validation
- All 256 Bitcoin opcodes (only 27 supported)
- Arithmetic opcodes (OP_ADD, OP_SUB, etc.)
- OP_CHECKMULTISIG
- Transaction serialization/deserialization
- SIGHASH computation from transaction data
- Taproot / SegWit witness evaluation
- P2SH (pay-to-script-hash) execution
- Script size/opcode count limits
- Locktime opcodes (OP_CHECKLOCKTIMEVERIFY, OP_CHECKSEQUENCEVERIFY)
- Any networking, async, or blockchain state

## Success Criteria

- [ ] `cargo build` compiles cleanly
- [ ] `cargo build --features secp256k1` compiles cleanly
- [ ] `cargo test` passes all 55+ tests
- [ ] `cargo test --features secp256k1` passes
- [ ] `cargo clippy` produces zero warnings
- [ ] `cargo fmt --check` passes
- [ ] `cargo doc --no-deps` builds without warnings
- [ ] All public functions have rustdoc comments
- [ ] No `unsafe`, no `unwrap()` in library code, no `panic!` in execution paths
- [ ] Examples compile and run
- [ ] Deterministic execution (no randomness, no timestamps, no I/O)

## Scope Lock

No feature creep. No CLI. No REST API. No database. No async. This is a library
crate. The scope is frozen after this plan is written.
