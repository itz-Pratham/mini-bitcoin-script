# mini-bitcoin-script

Minimal, protocol-accurate Bitcoin Script parsing and execution engine in Rust.

[![Crates.io](https://img.shields.io/crates/v/mini-bitcoin-script)](https://crates.io/crates/mini-bitcoin-script)
[![Docs.rs](https://docs.rs/mini-bitcoin-script/badge.svg)](https://docs.rs/mini-bitcoin-script)
[![License](https://img.shields.io/crates/l/mini-bitcoin-script)](LICENSE)

## What this crate does

- **Tokenizer** — Parses raw script bytes into tokens, handling all four push-data encodings (direct, PUSHDATA1/2/4)
- **Execution engine** — Stack-based VM executing 27 opcodes: constants, flow control, stack manipulation, comparison, logic, and crypto
- **P2PKH validation** — Protocol-accurate two-phase execution model (post-2010) for Pay-to-Public-Key-Hash scripts
- **Hash functions** — SHA-256, RIPEMD-160, HASH160 (RIPEMD160(SHA256)), HASH256 (SHA256(SHA256))
- **OP_CHECKSIG** — Stub mode by default; real ECDSA verification via optional `secp256k1` feature

## What this crate does NOT do

- Full consensus rule validation
- Arithmetic opcodes (OP_ADD, OP_SUB, etc.)
- OP_CHECKMULTISIG
- Transaction serialization or sighash computation
- SegWit, Taproot, or witness-based script types
- P2SH (pay-to-script-hash) execution
- Timelock opcodes (OP_CHECKLOCKTIMEVERIFY, OP_CHECKSEQUENCEVERIFY)
- Networking, async, or blockchain state

## Quick start

Add to your `Cargo.toml`:

```toml
[dependencies]
mini-bitcoin-script = "0.1"
```

Parse and execute a simple script:

```rust
use mini_bitcoin_script::tokenizer::parse_script_hex;
use mini_bitcoin_script::engine::execute;

// OP_1 OP_1 OP_EQUAL
let tokens = parse_script_hex("515187").unwrap();
let result = execute(&tokens).unwrap();
assert!(result); // 1 == 1
```

## P2PKH validation

```rust
use mini_bitcoin_script::hash;
use mini_bitcoin_script::script::validate_p2pkh;

let pubkey = b"fake-public-key-data";
let sig = b"fake-signature";
let pubkey_hash = hash::hash160(pubkey);

// Build scriptSig: <sig> <pubkey>
let mut script_sig = Vec::new();
script_sig.push(sig.len() as u8);
script_sig.extend_from_slice(sig);
script_sig.push(pubkey.len() as u8);
script_sig.extend_from_slice(pubkey);

// Build scriptPubKey: OP_DUP OP_HASH160 <hash> OP_EQUALVERIFY OP_CHECKSIG
let mut script_pubkey = vec![0x76, 0xa9, 0x14];
script_pubkey.extend_from_slice(&pubkey_hash);
script_pubkey.push(0x88);
script_pubkey.push(0xac);

let result = validate_p2pkh(&script_sig, &script_pubkey).unwrap();
assert!(result); // stub CHECKSIG always succeeds
```

## Feature flags

| Feature     | Description                                              |
|-------------|----------------------------------------------------------|
| `secp256k1` | Enables real ECDSA signature verification for OP_CHECKSIG via the `secp256k1` crate. Requires a sighash digest provided through `ExecuteOpts`. |

Enable with:

```toml
[dependencies]
mini-bitcoin-script = { version = "0.1", features = ["secp256k1"] }
```

## Examples

```sh
cargo run --example p2pkh    # Full P2PKH validation walkthrough
cargo run --example inspect  # Parse and display script tokens
```

## Supported opcodes

| Category       | Opcodes                                                             |
|----------------|---------------------------------------------------------------------|
| Constants      | OP_0, OP_1NEGATE, OP_1 through OP_16                               |
| Flow control   | OP_NOP, OP_IF, OP_NOTIF, OP_ELSE, OP_ENDIF, OP_VERIFY, OP_RETURN  |
| Stack          | OP_DUP, OP_DROP, OP_SWAP, OP_OVER, OP_NIP, OP_TUCK, OP_2DUP, OP_2DROP, OP_DEPTH, OP_SIZE |
| Comparison     | OP_EQUAL, OP_EQUALVERIFY                                           |
| Logic          | OP_NOT                                                              |
| Crypto         | OP_SHA256, OP_RIPEMD160, OP_HASH160, OP_HASH256, OP_CHECKSIG, OP_CHECKSIGVERIFY |

## Security disclaimer

**This crate is for educational and experimental use only.** It is not consensus-compatible with Bitcoin Core and must not be used to validate real transactions or protect real funds.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
