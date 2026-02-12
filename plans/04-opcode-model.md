# 04 — Opcode Model (`opcode.rs`)

## File: `src/opcode.rs`

## Opcode Enum

Fieldless enum. Derives `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`.

## Complete Opcode Table (27 opcodes)

### Constants (19 opcodes)

| Variant       | Byte   | Bitcoin Name    | Stack Effect                    |
|---------------|--------|-----------------|---------------------------------|
| `Op0`         | `0x00` | OP_0 / OP_FALSE | Push `vec![]` (empty = false)   |
| `Op1Negate`   | `0x4f` | OP_1NEGATE      | Push `vec![0x81]` (-1)          |
| `Op1`         | `0x51` | OP_1 / OP_TRUE  | Push `vec![0x01]`               |
| `Op2`         | `0x52` | OP_2            | Push `vec![0x02]`               |
| `Op3`         | `0x53` | OP_3            | Push `vec![0x03]`               |
| `Op4`         | `0x54` | OP_4            | Push `vec![0x04]`               |
| `Op5`         | `0x55` | OP_5            | Push `vec![0x05]`               |
| `Op6`         | `0x56` | OP_6            | Push `vec![0x06]`               |
| `Op7`         | `0x57` | OP_7            | Push `vec![0x07]`               |
| `Op8`         | `0x58` | OP_8            | Push `vec![0x08]`               |
| `Op9`         | `0x59` | OP_9            | Push `vec![0x09]`               |
| `Op10`        | `0x5a` | OP_10           | Push `vec![0x0a]`               |
| `Op11`        | `0x5b` | OP_11           | Push `vec![0x0b]`               |
| `Op12`        | `0x5c` | OP_12           | Push `vec![0x0c]`               |
| `Op13`        | `0x5d` | OP_13           | Push `vec![0x0d]`               |
| `Op14`        | `0x5e` | OP_14           | Push `vec![0x0e]`               |
| `Op15`        | `0x5f` | OP_15           | Push `vec![0x0f]`               |
| `Op16`        | `0x60` | OP_16           | Push `vec![0x10]`               |

### Flow Control (7 opcodes)

| Variant       | Byte   | Bitcoin Name | Stack Effect                        |
|---------------|--------|--------------|-------------------------------------|
| `OpNop`       | `0x61` | OP_NOP       | Do nothing                          |
| `OpIf`        | `0x63` | OP_IF        | Begin conditional block             |
| `OpNotIf`     | `0x64` | OP_NOTIF     | Begin inverted conditional block    |
| `OpElse`      | `0x67` | OP_ELSE      | Alternative branch                  |
| `OpEndIf`     | `0x68` | OP_ENDIF     | End conditional block               |
| `OpVerify`    | `0x69` | OP_VERIFY    | Fail if top is false, else pop      |
| `OpReturn`    | `0x6a` | OP_RETURN    | Immediately fail (unspendable)      |

### Stack Manipulation (9 opcodes)

| Variant       | Byte   | Bitcoin Name | Stack Effect                        |
|---------------|--------|--------------|-------------------------------------|
| `Op2Drop`     | `0x6d` | OP_2DROP     | Remove top two items                |
| `Op2Dup`      | `0x6e` | OP_2DUP      | Duplicate top two items             |
| `OpDepth`     | `0x74` | OP_DEPTH     | Push stack size as script integer   |
| `OpDrop`      | `0x75` | OP_DROP      | Remove top item                     |
| `OpDup`       | `0x76` | OP_DUP       | Duplicate top item                  |
| `OpNip`       | `0x77` | OP_NIP       | Remove second-from-top              |
| `OpOver`      | `0x78` | OP_OVER      | Copy second-from-top to top         |
| `OpSwap`      | `0x7c` | OP_SWAP      | Swap top two items                  |
| `OpTuck`      | `0x7d` | OP_TUCK      | Copy top, insert below second       |

### Splice (1 opcode)

| Variant       | Byte   | Bitcoin Name | Stack Effect                        |
|---------------|--------|--------------|-------------------------------------|
| `OpSize`      | `0x82` | OP_SIZE      | Push byte-length of top (no pop)    |

### Comparison (2 opcodes)

| Variant          | Byte   | Bitcoin Name    | Stack Effect                     |
|------------------|--------|-----------------|----------------------------------|
| `OpEqual`        | `0x87` | OP_EQUAL        | Pop 2, push 1 if equal else 0    |
| `OpEqualVerify`  | `0x88` | OP_EQUALVERIFY  | OP_EQUAL then OP_VERIFY          |

### Logic (1 opcode)

| Variant   | Byte   | Bitcoin Name | Stack Effect                          |
|-----------|--------|--------------|---------------------------------------|
| `OpNot`   | `0x91` | OP_NOT       | `[0]`/`[]` -> `[1]`; else -> `[]`     |

### Crypto (6 opcodes)

| Variant              | Byte   | Bitcoin Name       | Stack Effect               |
|----------------------|--------|--------------------|-----------------------------|
| `OpRipemd160`        | `0xa6` | OP_RIPEMD160       | Pop, push RIPEMD-160 (20B)  |
| `OpSha256`           | `0xa8` | OP_SHA256          | Pop, push SHA-256 (32B)     |
| `OpHash160`          | `0xa9` | OP_HASH160         | Pop, push SHA256+RIPEMD160  |
| `OpHash256`          | `0xaa` | OP_HASH256         | Pop, push double SHA-256    |
| `OpCheckSig`         | `0xac` | OP_CHECKSIG        | Pop pubkey+sig, verify      |
| `OpCheckSigVerify`   | `0xad` | OP_CHECKSIGVERIFY  | OP_CHECKSIG then OP_VERIFY  |

## Methods

```rust
/// Convert a byte to an Opcode, if it maps to a supported opcode.
/// Returns None for push-data bytes (0x01-0x4e), reserved opcodes,
/// and any unimplemented opcode.
pub fn from_byte(byte: u8) -> Option<Opcode>

/// Convert an Opcode back to its canonical byte value.
pub fn to_byte(&self) -> u8
```

## Display Implementation

Prints the standard Bitcoin name: `OP_DUP`, `OP_HASH160`, `OP_CHECKSIG`, etc.
`Op0` displays as `OP_0`, `Op1` displays as `OP_1`, etc.
