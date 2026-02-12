# 06 — Tokenizer (`tokenizer.rs`)

## File: `src/tokenizer.rs`

## Public API

```rust
/// Parse raw script bytes into a sequence of tokens.
pub fn parse_script(bytes: &[u8]) -> Result<Vec<Token>, ScriptError>

/// Parse a hex-encoded script string into tokens.
/// Convenience wrapper: decodes hex, then calls parse_script().
pub fn parse_script_hex(hex: &str) -> Result<Vec<Token>, ScriptError>
```

## Byte Dispatch Table

The tokenizer reads bytes left-to-right from position 0. For each byte:

| Byte(s)        | Action                                                         |
|----------------|----------------------------------------------------------------|
| `0x00`         | Emit `Token::Op(Opcode::Op0)`                                 |
| `0x01`-`0x4b`  | Direct push: byte value = data length N. Read N bytes, emit `Token::PushData(data)` |
| `0x4c`         | OP_PUSHDATA1: read 1 byte as length L, read L bytes, emit `Token::PushData` |
| `0x4d`         | OP_PUSHDATA2: read 2 bytes (little-endian) as length L, read L bytes, emit `Token::PushData` |
| `0x4e`         | OP_PUSHDATA4: read 4 bytes (little-endian) as length L, read L bytes, emit `Token::PushData` |
| `0x4f`         | Emit `Token::Op(Opcode::Op1Negate)`                           |
| `0x50`         | Return `Err(UnsupportedOpcode(0x50))` — OP_RESERVED           |
| `0x51`-`0x60`  | Emit `Token::Op(Opcode::Op1)` through `Token::Op(Opcode::Op16)` |
| `0x61`-`0xad`  | Look up via `Opcode::from_byte()`. If `Some`, emit `Token::Op`. If `None`, return `Err(UnsupportedOpcode(byte))` |
| `0xae`-`0xff`  | Return `Err(UnsupportedOpcode(byte))`                          |

## Push-Data Encoding Variants

Bitcoin has four distinct push-data mechanisms:

### 1. Direct Push (0x01-0x4b)
The byte value IS the data length. Next N bytes are the data.
```
[0x03, 0xaa, 0xbb, 0xcc]  ->  PushData([0xaa, 0xbb, 0xcc])
```

### 2. OP_PUSHDATA1 (0x4c)
Next 1 byte is the length (max 255 bytes).
```
[0x4c, 0x03, 0xaa, 0xbb, 0xcc]  ->  PushData([0xaa, 0xbb, 0xcc])
```

### 3. OP_PUSHDATA2 (0x4d)
Next 2 bytes (little-endian) are the length (max 65535 bytes).
```
[0x4d, 0x03, 0x00, 0xaa, 0xbb, 0xcc]  ->  PushData([0xaa, 0xbb, 0xcc])
```

### 4. OP_PUSHDATA4 (0x4e)
Next 4 bytes (little-endian) are the length.
```
[0x4e, 0x03, 0x00, 0x00, 0x00, 0xaa, 0xbb, 0xcc]  ->  PushData([0xaa, 0xbb, 0xcc])
```

## Error Conditions

| Condition                                  | Error                         |
|--------------------------------------------|-------------------------------|
| Push-data specifies L bytes, <L remain     | `UnexpectedEndOfScript`       |
| PUSHDATA1/2/4 but can't read length field  | `UnexpectedEndOfScript`       |
| Unrecognized/unimplemented opcode byte     | `UnsupportedOpcode(byte)`     |
| Empty input                                | `Ok(vec![])` — valid          |

## Implementation Notes

- Use a cursor index `pos: usize` walking through the byte slice
- Convert PUSHDATA2/4 length bytes with `u16::from_le_bytes` / `u32::from_le_bytes`
- For PUSHDATA4, convert length to `usize` (will not overflow on 32/64-bit)
- `parse_script_hex` calls `decode_hex()` then `parse_script()`
