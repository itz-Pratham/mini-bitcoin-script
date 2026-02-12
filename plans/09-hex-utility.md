# 09 — Hex Utility (`hex.rs`)

## File: `src/hex.rs`

## Public API

```rust
/// Decode a hexadecimal string into a byte vector.
///
/// Returns `Err(ScriptError::InvalidHex)` if the string has an odd number
/// of characters or contains non-hex characters.
///
/// Accepts both uppercase and lowercase hex digits. Does not accept
/// "0x" prefix — callers must strip it if present.
pub fn decode_hex(hex: &str) -> Result<Vec<u8>, ScriptError>
```

## Implementation

~15 lines. No external dependency. Parses character pairs using
`u8::from_str_radix(pair, 16)`.

```
fn decode_hex(hex: &str) -> Result<Vec<u8>, ScriptError>:
    if hex.len() % 2 != 0:
        return Err(InvalidHex)
    let mut bytes = Vec::with_capacity(hex.len() / 2)
    for chunk in hex.as_bytes().chunks(2):
        let pair = std::str::from_utf8(chunk)  // safe: already ASCII
        let byte = u8::from_str_radix(pair, 16).map_err(|_| InvalidHex)?
        bytes.push(byte)
    Ok(bytes)
```

## Edge Cases

| Input         | Result                      |
|---------------|-----------------------------|
| `""`          | `Ok(vec![])`                |
| `"00"`        | `Ok(vec![0x00])`            |
| `"ff"`        | `Ok(vec![0xff])`            |
| `"FF"`        | `Ok(vec![0xff])`            |
| `"aAbB"`      | `Ok(vec![0xaa, 0xbb])`     |
| `"0"`         | `Err(InvalidHex)` (odd)     |
| `"gg"`        | `Err(InvalidHex)` (invalid) |
| `"0x00"`      | `Err(InvalidHex)` (prefix)  |

## Why No External Dependency

Adding a `hex` crate for a single function that's 15 lines of code is
unnecessary. The implementation is trivial and has no edge cases beyond
what we handle. Keeps the dependency tree minimal.
