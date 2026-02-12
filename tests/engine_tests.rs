use mini_bitcoin_script::engine::execute;
use mini_bitcoin_script::error::ScriptError;
use mini_bitcoin_script::opcode::Opcode;
use mini_bitcoin_script::token::Token;
use mini_bitcoin_script::tokenizer::parse_script;

// ---------------------------------------------------------------------------
// Helper: build tokens from raw bytes via the tokenizer
// ---------------------------------------------------------------------------

fn run(bytes: &[u8]) -> Result<bool, ScriptError> {
    let tokens = parse_script(bytes)?;
    execute(&tokens)
}

fn run_tokens(tokens: &[Token]) -> Result<bool, ScriptError> {
    execute(tokens)
}

// ===========================================================================
// Stack Operations
// ===========================================================================

#[test]
fn op_dup_duplicates_top() {
    // push [0x42], OP_DUP, OP_EQUAL (equal to itself)
    let tokens = vec![
        Token::PushData(vec![0x42]),
        Token::Op(Opcode::OpDup),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_drop_removes_top() {
    // OP_1 OP_1 OP_DROP — stack has [1], which is truthy
    assert!(run(&[0x51, 0x51, 0x75]).unwrap());
}

#[test]
fn op_swap_two_elements() {
    // push A, push B, OP_SWAP, OP_DROP — drops B (now on top), leaves A
    let tokens = vec![
        Token::PushData(vec![0xaa]),
        Token::PushData(vec![0xbb]),
        Token::Op(Opcode::OpSwap),
        Token::Op(Opcode::OpDrop),
        // stack: [0xbb]
        // verify it's 0xbb by comparing with a fresh push
        Token::PushData(vec![0xbb]),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_over_copies_second() {
    // push A, push B, OP_OVER -> stack: A B A -> OP_DROP OP_DROP -> A
    let tokens = vec![
        Token::PushData(vec![0xaa]),
        Token::PushData(vec![0xbb]),
        Token::Op(Opcode::OpOver),
        // stack: [aa, bb, aa] — top is aa
        Token::PushData(vec![0xaa]),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_nip_removes_second() {
    // push A, push B, OP_NIP -> stack: [B]
    let tokens = vec![
        Token::PushData(vec![0xaa]),
        Token::PushData(vec![0xbb]),
        Token::Op(Opcode::OpNip),
        Token::PushData(vec![0xbb]),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_tuck_inserts_below_second() {
    // push A, push B, OP_TUCK -> stack: [B, A, B]
    // OP_DROP -> [B, A], OP_DROP -> [B]
    let tokens = vec![
        Token::PushData(vec![0xaa]),
        Token::PushData(vec![0xbb]),
        Token::Op(Opcode::OpTuck),
        Token::Op(Opcode::OpDrop),
        Token::Op(Opcode::OpDrop),
        Token::PushData(vec![0xbb]),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_2dup_duplicates_top_pair() {
    // push A, push B, OP_2DUP -> stack: [A, B, A, B]
    // OP_2DROP -> [A, B], OP_EQUAL (A == A from earlier? no — B == A is wrong)
    // Instead: verify by checking depth.
    // push A, push A, OP_2DUP -> [A, A, A, A], OP_EQUAL -> [A, A, 1]
    let tokens = vec![
        Token::PushData(vec![0xaa]),
        Token::PushData(vec![0xaa]),
        Token::Op(Opcode::Op2Dup),
        // stack: [aa, aa, aa, aa]
        Token::Op(Opcode::OpEqual), // top two: aa == aa -> true
        // stack: [aa, aa, 01]
        Token::Op(Opcode::OpDrop),  // remove the 01
        Token::Op(Opcode::OpEqual), // aa == aa -> true
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_2drop_removes_top_two() {
    // OP_1 OP_2 OP_3, OP_2DROP -> stack: [1]
    assert!(run(&[0x51, 0x52, 0x53, 0x6d]).unwrap());
}

#[test]
fn op_depth_pushes_count() {
    // OP_1 OP_1 OP_1 OP_DEPTH -> stack: [1,1,1,3]
    // OP_SWAP OP_DROP -> [1,1,3], check top is 3
    let tokens = vec![
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpDepth),
        // stack: [01, 01, 01, 03] — top is encoded 3
        Token::PushData(vec![0x03]),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_size_pushes_length() {
    // push [aa, bb], OP_SIZE -> stack: [[aa,bb], [02]]
    let tokens = vec![
        Token::PushData(vec![0xaa, 0xbb]),
        Token::Op(Opcode::OpSize),
        Token::PushData(vec![0x02]),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

// ===========================================================================
// Comparison & Logic
// ===========================================================================

#[test]
fn op_equal_true() {
    // OP_1 OP_1 OP_EQUAL
    assert!(run(&[0x51, 0x51, 0x87]).unwrap());
}

#[test]
fn op_equal_false() {
    // OP_1 OP_2 OP_EQUAL
    assert!(!run(&[0x51, 0x52, 0x87]).unwrap());
}

#[test]
fn op_equalverify_pass() {
    // OP_1 OP_1 OP_EQUALVERIFY OP_1
    assert!(run(&[0x51, 0x51, 0x88, 0x51]).unwrap());
}

#[test]
fn op_equalverify_fail() {
    // OP_1 OP_2 OP_EQUALVERIFY
    let err = run(&[0x51, 0x52, 0x88]).unwrap_err();
    assert_eq!(err, ScriptError::VerifyFailed);
}

#[test]
fn op_verify_true() {
    // OP_1 OP_VERIFY OP_1 (verify passes, OP_1 left on stack)
    assert!(run(&[0x51, 0x69, 0x51]).unwrap());
}

#[test]
fn op_verify_false() {
    // OP_0 OP_VERIFY
    let err = run(&[0x00, 0x69]).unwrap_err();
    assert_eq!(err, ScriptError::VerifyFailed);
}

#[test]
fn op_not_zero_becomes_one() {
    // OP_0 OP_NOT -> 1
    assert!(run(&[0x00, 0x91]).unwrap());
}

#[test]
fn op_not_one_becomes_zero() {
    // OP_1 OP_NOT -> 0
    assert!(!run(&[0x51, 0x91]).unwrap());
}

#[test]
fn op_not_five_becomes_zero() {
    // OP_5 OP_NOT -> 0
    assert!(!run(&[0x55, 0x91]).unwrap());
}

// ===========================================================================
// Flow Control
// ===========================================================================

#[test]
fn op_return_aborts() {
    // OP_RETURN
    let err = run(&[0x6a]).unwrap_err();
    assert_eq!(err, ScriptError::OpReturnEncountered);
}

#[test]
fn op_nop_has_no_effect() {
    // OP_1 OP_NOP -> true
    assert!(run(&[0x51, 0x61]).unwrap());
}

// ===========================================================================
// Crypto
// ===========================================================================

#[test]
fn op_sha256_known_vector() {
    // SHA256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    let expected =
        hex_literal::hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    let tokens = vec![
        Token::PushData(vec![]), // empty data
        Token::Op(Opcode::OpSha256),
        Token::PushData(expected.to_vec()),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_ripemd160_known_vector() {
    // RIPEMD160("") = 9c1185a5c5e9fc54612808977ee8f548b2258d31
    let expected = hex_literal::hex!("9c1185a5c5e9fc54612808977ee8f548b2258d31");
    let tokens = vec![
        Token::PushData(vec![]),
        Token::Op(Opcode::OpRipemd160),
        Token::PushData(expected.to_vec()),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_hash160_known_vector() {
    // HASH160("") = RIPEMD160(SHA256(""))
    //             = b472a266d0bd89c13706a4132ccfb16f7c3b9fcb
    let expected = hex_literal::hex!("b472a266d0bd89c13706a4132ccfb16f7c3b9fcb");
    let tokens = vec![
        Token::PushData(vec![]),
        Token::Op(Opcode::OpHash160),
        Token::PushData(expected.to_vec()),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_hash256_known_vector() {
    // HASH256("") = SHA256(SHA256(""))
    //             = 5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456
    let expected =
        hex_literal::hex!("5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456");
    let tokens = vec![
        Token::PushData(vec![]),
        Token::Op(Opcode::OpHash256),
        Token::PushData(expected.to_vec()),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

#[test]
fn op_checksig_stub_succeeds() {
    // Push fake sig and pubkey, OP_CHECKSIG (stub mode)
    let tokens = vec![
        Token::PushData(vec![0x30; 71]), // fake DER sig
        Token::PushData(vec![0x02; 33]), // fake compressed pubkey
        Token::Op(Opcode::OpCheckSig),
    ];
    assert!(run_tokens(&tokens).unwrap());
}

// ===========================================================================
// Edge Cases
// ===========================================================================

#[test]
fn empty_token_list_returns_false() {
    assert!(!run_tokens(&[]).unwrap());
}

#[test]
fn op1_alone_is_true() {
    // OP_1
    assert!(run(&[0x51]).unwrap());
}

#[test]
fn op0_alone_is_false() {
    // OP_0
    assert!(!run(&[0x00]).unwrap());
}

#[test]
fn stack_underflow_dup_empty() {
    // OP_DUP on empty stack
    let err = run(&[0x76]).unwrap_err();
    assert_eq!(err, ScriptError::StackUnderflow);
}

#[test]
fn stack_underflow_drop_empty() {
    // OP_DROP on empty stack
    let err = run(&[0x75]).unwrap_err();
    assert_eq!(err, ScriptError::StackUnderflow);
}

#[test]
fn stack_underflow_swap_one_item() {
    // OP_1 OP_SWAP
    let err = run(&[0x51, 0x7c]).unwrap_err();
    assert_eq!(err, ScriptError::StackUnderflow);
}

#[test]
fn stack_underflow_equal_one_item() {
    // OP_1 OP_EQUAL
    let err = run(&[0x51, 0x87]).unwrap_err();
    assert_eq!(err, ScriptError::StackUnderflow);
}

#[test]
fn op_depth_empty_stack() {
    // OP_DEPTH on empty stack -> pushes 0 (encoded as [])
    assert!(!run(&[0x74]).unwrap());
}

#[test]
fn op_1negate_pushes_0x81() {
    let tokens = vec![
        Token::Op(Opcode::Op1Negate),
        Token::PushData(vec![0x81]),
        Token::Op(Opcode::OpEqual),
    ];
    assert!(run_tokens(&tokens).unwrap());
}
