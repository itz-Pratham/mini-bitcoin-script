# 14 — Examples

## examples/p2pkh.rs

Demonstrates the complete P2PKH validation flow.

### What it does:
1. Creates a fake signature (71 bytes) and compressed public key (33 bytes)
2. Computes HASH160 of the public key using `hash::hash160()`
3. Builds scriptSig and scriptPubKey as raw bytes
4. Calls `validate_p2pkh()` and prints the result
5. Also parses and displays the scriptPubKey tokens for visual inspection

### Expected output:
```
scriptPubKey tokens:
  OP_DUP OP_HASH160 <...20 bytes...> OP_EQUALVERIFY OP_CHECKSIG

P2PKH validation result: true
```

Run with: `cargo run --example p2pkh`

---

## examples/inspect.rs

Demonstrates script parsing and display.

### What it does:
1. Takes a hardcoded hex-encoded P2PKH scriptPubKey
   (can be a real one from the Bitcoin blockchain)
2. Parses with `parse_script_hex()`
3. Prints each token using Display formatting
4. Shows raw bytes vs. human-readable representation

### Expected output:
```
Raw hex: 76a91489abcdefabbaabbaabbaabbaabbaabbaabbaabba88ac
Parsed tokens:
  [0] OP_DUP
  [1] OP_HASH160
  [2] <89abcdefabbaabbaabbaabbaabbaabbaabbaabba>
  [3] OP_EQUALVERIFY
  [4] OP_CHECKSIG
```

Run with: `cargo run --example inspect`

---

## Example Construction Notes

- Examples must NOT use `unwrap()` — use `expect()` with descriptive messages,
  or pattern match on Result. Since these are examples (not library code),
  `expect()` is acceptable.
- Examples should demonstrate realistic Bitcoin Script patterns.
- Keep examples short and focused. Each one demonstrates a single concept.
