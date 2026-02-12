//! Demonstrates script parsing and human-readable display.
//!
//! Run with: `cargo run --example inspect`

use mini_bitcoin_script::tokenizer::parse_script_hex;

fn main() {
    // A standard P2PKH scriptPubKey in hex:
    // OP_DUP OP_HASH160 <20-byte pubkey hash> OP_EQUALVERIFY OP_CHECKSIG
    let hex = "76a91489abcdefabbaabbaabbaabbaabbaabbaabbaabba88ac";

    println!("Raw hex: {hex}");
    println!();

    let tokens = parse_script_hex(hex).expect("valid hex script");

    println!("Parsed tokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!("  [{i}] {token}");
    }
}
