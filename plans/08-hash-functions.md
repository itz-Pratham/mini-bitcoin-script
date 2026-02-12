# 08 — Hash Functions (`hash.rs`)

## File: `src/hash.rs`

## Public API

```rust
/// Compute the SHA-256 hash of the input data.
pub fn sha256(data: &[u8]) -> [u8; 32]

/// Compute the RIPEMD-160 hash of the input data.
pub fn ripemd160(data: &[u8]) -> [u8; 20]

/// Compute HASH160: SHA-256 followed by RIPEMD-160.
/// This is the standard Bitcoin address hash function.
pub fn hash160(data: &[u8]) -> [u8; 20]

/// Compute HASH256: double SHA-256 (SHA-256 of SHA-256).
/// This is the standard Bitcoin transaction/block hash function.
pub fn hash256(data: &[u8]) -> [u8; 32]
```

## Implementation Details

All functions are infallible. Hashing cannot fail. No `Result` return type.

Uses:
- `sha2::Sha256` from the `sha2` crate (RustCrypto)
- `ripemd::Ripemd160` from the `ripemd` crate (RustCrypto)

Both use the `Digest` trait pattern:
```rust
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
```

## Hash Composition

- `hash160(data)` = `ripemd160(sha256(data))`
  - SHA-256 the data first (32 bytes), then RIPEMD-160 the result (20 bytes)
  - Used in P2PKH: hash the public key to get the address

- `hash256(data)` = `sha256(sha256(data))`
  - Double SHA-256
  - Used in Bitcoin for transaction hashes and block hashes

## Known Test Vectors

### SHA-256 of empty string
```
Input:  b""
Output: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

### RIPEMD-160 of empty string
```
Input:  b""
Output: 9c1185a5c5e9fc54612808977ee8f548b2258d31
```

### HASH160 of empty string
```
Input:  b""
SHA256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
Output: b472a266d0bd89c13706a4132ccfb16f7c3b9fcb
```

### HASH256 of empty string
```
Input:  b""
SHA256: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
Output: 5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456
```

These vectors will be used in integration tests.
