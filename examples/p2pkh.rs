//! Demonstrates the complete Pay-to-Public-Key-Hash (P2PKH) validation flow.
//!
//! Run with: `cargo run --example p2pkh`

use mini_bitcoin_script::hash;
use mini_bitcoin_script::script::validate_p2pkh;
use mini_bitcoin_script::tokenizer::parse_script;

fn main() {
    // 1. Create a fake 71-byte signature and 33-byte compressed public key.
    //    In a real transaction these would come from the spending input.
    let fake_sig = [0x30u8; 71];
    let fake_pubkey = [0x02u8; 33];

    // 2. Compute HASH160(pubkey) — this is the 20-byte "address hash" that
    //    appears in the locking script.
    let pubkey_hash = hash::hash160(&fake_pubkey);

    // 3. Build scriptSig: <sig> <pubkey>
    let mut script_sig = Vec::new();
    script_sig.push(fake_sig.len() as u8); // direct push length
    script_sig.extend_from_slice(&fake_sig);
    script_sig.push(fake_pubkey.len() as u8);
    script_sig.extend_from_slice(&fake_pubkey);

    // 4. Build scriptPubKey: OP_DUP OP_HASH160 <20-byte hash> OP_EQUALVERIFY OP_CHECKSIG
    let mut script_pubkey = Vec::new();
    script_pubkey.push(0x76); // OP_DUP
    script_pubkey.push(0xa9); // OP_HASH160
    script_pubkey.push(0x14); // push 20 bytes
    script_pubkey.extend_from_slice(&pubkey_hash);
    script_pubkey.push(0x88); // OP_EQUALVERIFY
    script_pubkey.push(0xac); // OP_CHECKSIG

    // 5. Display the scriptPubKey tokens for inspection.
    let tokens = parse_script(&script_pubkey).expect("valid scriptPubKey");
    println!("scriptPubKey tokens:");
    print!(" ");
    for token in &tokens {
        print!(" {token}");
    }
    println!();
    println!();

    // 6. Validate the P2PKH script pair (stub CHECKSIG mode).
    let result = validate_p2pkh(&script_sig, &script_pubkey).expect("execution succeeded");
    println!("P2PKH validation result: {result}");
}
