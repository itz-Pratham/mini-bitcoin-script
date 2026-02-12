# 07 — Stack (`stack.rs`)

## File: `src/stack.rs`

## Visibility

`pub(crate)` — the Stack type is NOT part of the public API. Internal to the
crate, used only by `engine.rs`.

## Structure

```rust
pub(crate) struct Stack {
    items: Vec<Vec<u8>>,
}
```

## Methods

| Method      | Signature                                         | Notes                          |
|-------------|---------------------------------------------------|--------------------------------|
| `new`       | `() -> Self`                                      | Empty stack                    |
| `push`      | `(&mut self, item: Vec<u8>)`                      | Infallible                     |
| `pop`       | `(&mut self) -> Result<Vec<u8>, ScriptError>`     | `StackUnderflow` if empty      |
| `peek`      | `(&self) -> Result<&[u8], ScriptError>`           | View top without removing      |
| `len`       | `(&self) -> usize`                                |                                |
| `is_empty`  | `(&self) -> bool`                                 |                                |
| `push_bool` | `(&mut self, val: bool)`                          | true -> `[1]`, false -> `[]`   |
| `remove`    | `(&mut self, idx: usize) -> Result<Vec<u8>, _>`   | For OP_NIP (remove by index)   |

## Bitcoin Truthiness Function

```rust
pub(crate) fn is_true(bytes: &[u8]) -> bool
```

Bitcoin defines false as any representation of zero:

- Empty byte vector `[]` is false
- Any byte vector where all bytes are `0x00`, except possibly the last byte
  which may be `0x80` (negative-zero sign bit), is false
- Everything else is true

### Truth Table

| Input                  | Result | Reason                   |
|------------------------|--------|--------------------------|
| `[]`                   | false  | Empty                    |
| `[0x00]`               | false  | Zero                     |
| `[0x80]`               | false  | Negative zero            |
| `[0x00, 0x00]`         | false  | Multi-byte zero          |
| `[0x00, 0x80]`         | false  | Multi-byte negative zero |
| `[0x00, 0x00, 0x80]`   | false  | Multi-byte negative zero |
| `[0x01]`               | true   | Non-zero                 |
| `[0x81]`               | true   | -1 (non-zero)            |
| `[0x00, 0x01]`         | true   | Non-zero                 |
| `[0x80, 0x00]`         | true   | Non-zero (0x80 not last) |

### Implementation Logic

```
fn is_true(bytes: &[u8]) -> bool:
    if bytes.is_empty():
        return false
    // Check all bytes except last are 0x00
    for byte in &bytes[..bytes.len() - 1]:
        if *byte != 0x00:
            return true
    // Last byte: 0x00 or 0x80 means false
    let last = bytes[bytes.len() - 1];
    last != 0x00 && last != 0x80
```

This matches the Bitcoin Core implementation exactly: "False is represented by
any representation of zero and True is represented by any representation of
non-zero."

## Stack Index Convention

The stack is a `Vec<Vec<u8>>` where:
- `items.last()` = top of stack (most recently pushed)
- `items[0]` = bottom of stack

`push` appends to the end. `pop` removes from the end.
For `remove(idx)`, the index is from the Vec perspective (0 = bottom).
For OP_NIP, we need to remove the second-from-top, which is
`items.len() - 2`.
