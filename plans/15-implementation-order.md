# 15 — Implementation Order

Each step must compile and pass tests before proceeding to the next.

## Step Sequence

| Step | Module           | What It Unlocks                                    |
|------|------------------|----------------------------------------------------|
| 1    | `Cargo.toml`     | Dependencies, metadata, feature flag, edition 2021 |
| 2    | `src/error.rs`   | Every other module depends on ScriptError          |
| 3    | `src/hex.rs`     | Needed by tokenizer and tests                      |
| 4    | `src/opcode.rs`  | Needed by token and tokenizer                      |
| 5    | `src/token.rs`   | Needed by tokenizer and engine                     |
| 6    | `src/hash.rs`    | Needed by engine (crypto opcodes)                  |
| 7    | `src/stack.rs`   | Needed by engine                                   |
| 8    | `src/tokenizer.rs`| First integration point — can parse real scripts  |
| 9    | `src/engine.rs`  | Core VM — the largest module                       |
| 10   | `src/script.rs`  | P2PKH validation — ties it all together            |
| 11   | `src/lib.rs`     | Wire all modules, crate-level docs, re-exports     |
| 12   | `tests/`         | All integration test files                         |
| 13   | `examples/`      | `p2pkh.rs`, `inspect.rs`                           |
| 14   | `README.md`      | Usage, scope, disclaimers, badges                  |
| 15   | Final polish     | `cargo clippy`, `cargo fmt`, `cargo test`, review  |

## Verification After Each Step

After each step, run:
```bash
cargo build     # Must compile
cargo test      # Must pass all existing tests
cargo clippy    # Must have zero warnings
```

After step 11 (all modules wired):
```bash
cargo build --features secp256k1   # Feature flag must work
```

After step 15 (final polish):
```bash
cargo fmt --check
cargo doc --no-deps
cargo test --features secp256k1
```

## Why This Order

The order follows the dependency graph bottom-up:
1. Leaf modules first (error, hex, opcode) — no dependencies
2. Modules that depend on leaves next (token, hash, stack)
3. Integration modules last (tokenizer, engine, script)
4. Wiring and testing after all code exists

This ensures every `cargo build` succeeds after every step. No forward
references, no stub modules needed.
