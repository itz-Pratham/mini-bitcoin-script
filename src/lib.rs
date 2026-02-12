//! Minimal Bitcoin Script parsing and execution engine, built for
//! education and tooling.
//!
//! Bitcoin Script is the stack-based programming language used to define
//! spending conditions for Bitcoin transaction outputs. This crate implements
//! a subset of the Script instruction set sufficient for understanding and
//! validating common transaction patterns, including Pay-to-Public-Key-Hash
//! (P2PKH).
//!
//! # Consensus warning
//!
//! **This crate is NOT consensus-compatible with Bitcoin Core.**
//!
//! A reimplementation of Bitcoin Script cannot guarantee identical
//! behavior to Bitcoin Core's C++ interpreter across all edge cases —
//! number encoding boundaries, error ordering, stack overflow semantics,
//! and other subtle behaviors that define Bitcoin's consensus rules.
//!
//! **Do not use this crate to validate real transactions or protect real
//! funds.**
//!
//! This crate is intended for:
//! - **Education** — learning how Bitcoin Script works internally
//! - **Tooling** — script inspection, debugging, and construction
//! - **Testing** — validating script logic before broadcast
//!
//! # What this crate implements
//!
//! - **Tokenizer**: Parses raw script bytes into a sequence of [`token::Token`]s,
//!   handling all four push-data encodings (direct, PUSHDATA1/2/4).
//! - **Execution engine**: A stack-based virtual machine that executes
//!   tokenized scripts with support for 27 opcodes including conditionals,
//!   stack manipulation, comparison, hashing, and signature verification.
//! - **P2PKH validation**: Protocol-accurate two-phase execution model
//!   (post-2010) for Pay-to-Public-Key-Hash scripts.
//! - **Hash functions**: SHA-256, RIPEMD-160, HASH160, and HASH256.
//!
//! # What is NOT implemented
//!
//! - Arithmetic opcodes (OP_ADD, OP_SUB, etc.)
//! - Multi-signature opcodes (OP_CHECKMULTISIG)
//! - Timelock opcodes (OP_CHECKLOCKTIMEVERIFY, OP_CHECKSEQUENCEVERIFY)
//! - SegWit, Taproot, or any witness-based script types
//! - Transaction serialization or sighash computation
//!
//! # OP_CHECKSIG behavior
//!
//! By default, `OP_CHECKSIG` operates in **stub mode**: it pops two stack
//! elements (pubkey and signature) and always pushes `true`. This allows
//! testing script logic without real cryptographic keys.
//!
//! With the `secp256k1` Cargo feature enabled and a pre-computed sighash
//! provided via [`engine::ExecuteOpts`], real ECDSA signature verification
//! is performed using the `secp256k1` crate.
//!
//! # Security disclaimer
//!
//! **This crate is NOT consensus-compatible with Bitcoin Core.** It must
//! not be used to validate real transactions or protect real funds.
//!
//! # Quick example
//!
//! ```rust
//! use mini_bitcoin_script::tokenizer::parse_script_hex;
//! use mini_bitcoin_script::engine::execute;
//!
//! // Simple script: OP_1 OP_1 OP_EQUAL
//! let tokens = parse_script_hex("515187").unwrap();
//! let result = execute(&tokens).unwrap();
//! assert!(result); // 1 == 1
//! ```
//!
//! # Feature flags
//!
//! | Feature     | Description                                        |
//! |-------------|----------------------------------------------------|
//! | `secp256k1` | Enables real ECDSA signature verification for      |
//! |             | OP_CHECKSIG via the `secp256k1` crate.             |

pub mod engine;
pub mod error;
pub mod hash;
pub mod hex;
pub mod opcode;
pub mod script;
pub(crate) mod stack;
pub mod token;
pub mod tokenizer;
