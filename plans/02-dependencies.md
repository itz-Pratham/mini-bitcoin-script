# 02 — Dependencies & Cargo.toml

## Complete Cargo.toml

```toml
[package]
name = "mini-bitcoin-script"
version = "0.1.0"
edition = "2021"
rust-version = "1.63"
description = "Minimal, protocol-accurate Bitcoin Script parsing and execution engine"
license = "MIT OR Apache-2.0"
repository = "https://github.com/itz-Pratham/mini-bitcoin-script"
documentation = "https://docs.rs/mini-bitcoin-script"
keywords = ["bitcoin", "script", "blockchain", "crypto", "p2pkh"]
categories = ["cryptography", "parser-implementations"]
readme = "README.md"

[dependencies]
sha2 = "0.10"
ripemd = "0.1"

[dependencies.secp256k1]
version = "0.29"
optional = true
features = ["global-context"]

[dev-dependencies]
hex-literal = "0.4"

[features]
default = []
secp256k1 = ["dep:secp256k1"]
```

## Dependency Rationale

| Crate         | Type      | Purpose                                      |
|---------------|-----------|----------------------------------------------|
| `sha2`        | Required  | SHA-256 for OP_SHA256, OP_HASH160, OP_HASH256|
| `ripemd`      | Required  | RIPEMD-160 for OP_RIPEMD160, OP_HASH160      |
| `secp256k1`   | Optional  | Real ECDSA verification for OP_CHECKSIG      |
| `hex-literal` | Dev-only  | `hex!()` macro for test byte arrays          |

All dependencies are free, open-source Rust crates from the RustCrypto project
(sha2, ripemd) or Bitcoin Core bindings (secp256k1). Zero monetary cost.

## Feature Flags

| Feature     | Effect                                             |
|-------------|----------------------------------------------------|
| `secp256k1` | Enables real OP_CHECKSIG ECDSA verification.       |
|             | Without: OP_CHECKSIG always pushes true (stub).    |
|             | With: real verification when sighash is provided.  |

## Licensing

Dual-licensed under MIT OR Apache-2.0. This is the standard Rust ecosystem
convention. Two license files:
- `LICENSE` (MIT) — already exists
- `LICENSE-APACHE` (Apache-2.0) — to be added
