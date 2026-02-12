# 05 — Token Model (`token.rs`)

## File: `src/token.rs`

## Token Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Data pushed onto the stack by a push-data instruction.
    /// Covers: direct push (0x01-0x4b), OP_PUSHDATA1, OP_PUSHDATA2, OP_PUSHDATA4.
    PushData(Vec<u8>),

    /// An opcode instruction (any non-push operation).
    Op(Opcode),
}
```

## Design Rationale

The `Token` type is the output of the tokenizer and the input to the engine.
It separates "what opcode" from "what data" cleanly:

- `Op(Opcode)` — an instruction to execute
- `PushData(Vec<u8>)` — data to push onto the stack

This avoids having a `Push(Vec<u8>)` variant inside the fieldless `Opcode` enum,
which would prevent `Opcode` from deriving `Copy`.

## Display Implementation

- `Token::PushData(data)` prints as `<hex_encoded_data>`
  - Example: `<89abcdef01234567>`
  - Empty data: `<>`
- `Token::Op(opcode)` delegates to `Opcode::fmt()`
  - Example: `OP_DUP`

## Formatted P2PKH scriptPubKey Example

```
OP_DUP OP_HASH160 <89abcdefabbaabbaabbaabbaabbaabbaabbaabba> OP_EQUALVERIFY OP_CHECKSIG
```

## Usage in Engine

The engine iterates over `&[Token]` and matches:

```rust
match token {
    Token::PushData(data) => stack.push(data.clone()),
    Token::Op(opcode) => { /* dispatch to opcode handler */ }
}
```
