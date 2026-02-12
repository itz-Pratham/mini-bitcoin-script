# 13 — Test Plan

## Test Organization

- **Unit tests**: In-module `#[cfg(test)]` blocks for internal behavior
- **Integration tests**: `tests/` directory for public API behavior

All integration test files use `hex_literal::hex!` for byte arrays.

---

## tests/stack_tests.rs

Tests for the stack (accessed via engine execution of crafted token sequences,
since Stack is pub(crate)).

| # | Test Case                              | Expected Result       |
|---|----------------------------------------|-----------------------|
| 1 | `is_true` with `[]`                    | false                 |
| 2 | `is_true` with `[0x00]`               | false                 |
| 3 | `is_true` with `[0x80]`               | false (negative zero) |
| 4 | `is_true` with `[0x00, 0x80]`         | false                 |
| 5 | `is_true` with `[0x00, 0x00]`         | false                 |
| 6 | `is_true` with `[0x00, 0x00, 0x80]`   | false                 |
| 7 | `is_true` with `[0x01]`               | true                  |
| 8 | `is_true` with `[0x81]`               | true (-1)             |
| 9 | `is_true` with `[0x00, 0x01]`         | true                  |
| 10| `is_true` with `[0x80, 0x00]`         | true (0x80 not last)  |

Note: Since `is_true` and `Stack` are pub(crate), these tests go in a unit
test module inside `stack.rs` rather than in `tests/`.

---

## tests/tokenizer_tests.rs

| # | Test Case                              | Expected Result                    |
|---|----------------------------------------|------------------------------------|
| 1 | Empty script `[]`                      | `Ok(vec![])`                       |
| 2 | Single opcode `[0x76]`                 | `[Op(OpDup)]`                      |
| 3 | Direct push `[0x03, 0xaa, 0xbb, 0xcc]`| `[PushData([aa,bb,cc])]`          |
| 4 | OP_PUSHDATA1 `[0x4c, 0x03, aa, bb, cc]`| `[PushData([aa,bb,cc])]`         |
| 5 | OP_PUSHDATA2 `[0x4d, 03, 00, aa, bb, cc]`| `[PushData([aa,bb,cc])]`       |
| 6 | OP_PUSHDATA4 `[0x4e, 03, 00, 00, 00, aa, bb, cc]`| `[PushData]`          |
| 7 | Truncated push data `[0x03, 0xaa]`     | `Err(UnexpectedEndOfScript)`      |
| 8 | OP_0 `[0x00]`                          | `[Op(Op0)]`                        |
| 9 | OP_1 through OP_16                     | Correct opcode variants            |
| 10| P2PKH scriptPubKey `76 a9 14 <20B> 88 ac` | Correct token sequence        |
| 11| Unsupported opcode `[0xb0]`            | `Err(UnsupportedOpcode(0xb0))`    |
| 12| `parse_script_hex` with valid hex      | Same as parse_script               |
| 13| `parse_script_hex` with invalid hex    | `Err(InvalidHex)`                  |
| 14| OP_PUSHDATA1 truncated length          | `Err(UnexpectedEndOfScript)`      |
| 15| Zero-length direct push `[0x00]`       | `[Op(Op0)]` (OP_0, not push)      |
| 16| OP_1NEGATE `[0x4f]`                    | `[Op(Op1Negate)]`                  |

---

## tests/engine_tests.rs

### Stack Operations
| # | Test Case                              | Expected Result       |
|---|----------------------------------------|-----------------------|
| 1 | OP_DUP: push, dup, verify 2 copies     | Two identical items   |
| 2 | OP_DROP: push, drop, verify empty       | Empty stack           |
| 3 | OP_SWAP: push A B, swap, verify B A     | Swapped order         |
| 4 | OP_OVER: push A B, over, verify A B A   | Copy of second        |
| 5 | OP_NIP: push A B, nip, verify B         | Second removed        |
| 6 | OP_TUCK: push A B, tuck, verify B A B   | Top tucked below      |
| 7 | OP_2DUP: push A B, 2dup, verify 4 items | Duplicated pair       |
| 8 | OP_2DROP: push A B, 2drop, verify empty | Two items removed     |
| 9 | OP_DEPTH: push 3 items, depth           | Top is `[0x03]`       |
| 10| OP_SIZE: push `[aa,bb]`, size           | Top is `[0x02]`       |

### Comparison & Logic
| # | Test Case                              | Expected Result       |
|---|----------------------------------------|-----------------------|
| 11| OP_EQUAL: equal items                   | `Ok(true)`            |
| 12| OP_EQUAL: unequal items                 | `Ok(false)`           |
| 13| OP_EQUALVERIFY: equal                   | Continues             |
| 14| OP_EQUALVERIFY: unequal                 | `Err(VerifyFailed)`   |
| 15| OP_VERIFY: true value                   | Continues             |
| 16| OP_VERIFY: false value                  | `Err(VerifyFailed)`   |
| 17| OP_NOT: `[0]` -> `[1]`                 | true                  |
| 18| OP_NOT: `[1]` -> `[]`                  | false                 |
| 19| OP_NOT: `[5]` -> `[]`                  | false                 |

### Flow Control
| # | Test Case                              | Expected Result       |
|---|----------------------------------------|-----------------------|
| 20| OP_RETURN                               | `Err(OpReturnEncountered)` |
| 21| OP_NOP                                  | No effect             |

### Crypto
| # | Test Case                              | Expected Result       |
|---|----------------------------------------|-----------------------|
| 22| OP_SHA256: hash of empty string         | Known test vector     |
| 23| OP_RIPEMD160: hash of empty string      | Known test vector     |
| 24| OP_HASH160: hash of empty string        | Known test vector     |
| 25| OP_HASH256: hash of empty string        | Known test vector     |
| 26| OP_CHECKSIG stub: push sig+pubkey       | `Ok(true)`            |

### Edge Cases
| # | Test Case                              | Expected Result       |
|---|----------------------------------------|-----------------------|
| 27| Empty token list                        | `Ok(false)` (empty)   |
| 28| OP_1 alone                              | `Ok(true)`            |
| 29| OP_0 alone                              | `Ok(false)`           |
| 30| Stack underflow: OP_DUP on empty        | `Err(StackUnderflow)` |
| 31| Stack underflow: OP_DROP on empty       | `Err(StackUnderflow)` |
| 32| Stack underflow: OP_SWAP with 1 item    | `Err(StackUnderflow)` |
| 33| Stack underflow: OP_EQUAL with 1 item   | `Err(StackUnderflow)` |
| 34| OP_DEPTH on empty stack                 | Stack has `[]` (zero) |
| 35| OP_1NEGATE                              | `[0x81]` on stack     |

---

## tests/conditionals_tests.rs

| # | Test Case                                              | Expected Result         |
|---|--------------------------------------------------------|-------------------------|
| 1 | `OP_1 OP_IF OP_1 OP_ENDIF`                            | `Ok(true)`              |
| 2 | `OP_0 OP_IF OP_1 OP_ENDIF`                            | `Ok(false)` (skipped)   |
| 3 | `OP_1 OP_IF OP_1 OP_ELSE OP_0 OP_ENDIF`              | `Ok(true)` (true branch)|
| 4 | `OP_0 OP_IF OP_1 OP_ELSE OP_0 OP_ENDIF`              | `Ok(false)` (else)      |
| 5 | `OP_1 OP_NOTIF OP_1 OP_ELSE OP_0 OP_ENDIF`           | `Ok(false)` (inverted)  |
| 6 | `OP_0 OP_NOTIF OP_1 OP_ELSE OP_0 OP_ENDIF`           | `Ok(true)` (inverted)   |
| 7 | Nested: `OP_1 OP_IF OP_1 OP_IF OP_1 OP_ENDIF OP_ENDIF`| `Ok(true)`             |
| 8 | `OP_IF` without `OP_ENDIF`                             | `Err(UnbalancedConditional)` |
| 9 | `OP_ENDIF` without `OP_IF`                             | `Err(UnbalancedConditional)` |
| 10| `OP_ELSE` without `OP_IF`                              | `Err(UnbalancedConditional)` |
| 11| 3-level nesting with mixed true/false                   | Correct branch execution |
| 12| False outer, true inner (inner should still skip)       | Correct skip behavior   |

---

## tests/p2pkh_tests.rs

| # | Test Case                              | Expected Result                    |
|---|----------------------------------------|------------------------------------|
| 1 | Valid P2PKH: correct pubkey hash       | `Ok(true)` (stub CHECKSIG)         |
| 2 | Invalid P2PKH: wrong pubkey hash       | `Err(VerifyFailed)`                |
| 3 | Empty scriptSig                        | `Err(StackUnderflow)`              |
| 4 | Malformed scriptPubKey bytes           | Appropriate parse error            |

### Valid P2PKH Test Construction
1. Create fake 71-byte signature
2. Create fake 33-byte compressed public key
3. Compute `hash160(pubkey)` -> 20-byte hash
4. Build scriptSig: `<sig> <pubkey>` as raw bytes
5. Build scriptPubKey: `OP_DUP OP_HASH160 <hash> OP_EQUALVERIFY OP_CHECKSIG`
6. Call `validate_p2pkh(script_sig, script_pubkey)`
7. Assert `Ok(true)`
