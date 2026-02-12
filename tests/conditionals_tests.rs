use mini_bitcoin_script::engine::execute;
use mini_bitcoin_script::error::ScriptError;
use mini_bitcoin_script::opcode::Opcode;
use mini_bitcoin_script::token::Token;

fn run(tokens: &[Token]) -> Result<bool, ScriptError> {
    execute(tokens)
}

// ---------------------------------------------------------------------------
// Basic IF / ELSE / ENDIF
// ---------------------------------------------------------------------------

#[test]
fn if_true_executes_body() {
    // OP_1 OP_IF OP_1 OP_ENDIF
    let tokens = vec![
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(run(&tokens).unwrap());
}

#[test]
fn if_false_skips_body() {
    // OP_0 OP_IF OP_1 OP_ENDIF -> empty stack -> false
    let tokens = vec![
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(!run(&tokens).unwrap());
}

#[test]
fn if_true_else_takes_true_branch() {
    // OP_1 OP_IF OP_1 OP_ELSE OP_0 OP_ENDIF
    let tokens = vec![
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpElse),
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(run(&tokens).unwrap());
}

#[test]
fn if_false_else_takes_else_branch() {
    // OP_0 OP_IF OP_1 OP_ELSE OP_0 OP_ENDIF -> 0 (false)
    let tokens = vec![
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpElse),
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(!run(&tokens).unwrap());
}

#[test]
fn notif_true_skips_body() {
    // OP_1 OP_NOTIF OP_1 OP_ELSE OP_0 OP_ENDIF -> 0 (else branch)
    let tokens = vec![
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpNotIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpElse),
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(!run(&tokens).unwrap());
}

#[test]
fn notif_false_executes_body() {
    // OP_0 OP_NOTIF OP_1 OP_ELSE OP_0 OP_ENDIF -> 1 (true branch)
    let tokens = vec![
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpNotIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpElse),
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(run(&tokens).unwrap());
}

#[test]
fn nested_if_both_true() {
    // OP_1 OP_IF OP_1 OP_IF OP_1 OP_ENDIF OP_ENDIF
    let tokens = vec![
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpEndIf),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(run(&tokens).unwrap());
}

// ---------------------------------------------------------------------------
// Unbalanced conditionals
// ---------------------------------------------------------------------------

#[test]
fn if_without_endif() {
    let tokens = vec![
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
    ];
    let err = run(&tokens).unwrap_err();
    assert_eq!(err, ScriptError::UnbalancedConditional);
}

#[test]
fn endif_without_if() {
    let tokens = vec![Token::Op(Opcode::OpEndIf)];
    let err = run(&tokens).unwrap_err();
    assert_eq!(err, ScriptError::UnbalancedConditional);
}

#[test]
fn else_without_if() {
    let tokens = vec![Token::Op(Opcode::OpElse)];
    let err = run(&tokens).unwrap_err();
    assert_eq!(err, ScriptError::UnbalancedConditional);
}

// ---------------------------------------------------------------------------
// Deep nesting
// ---------------------------------------------------------------------------

#[test]
fn three_level_nesting() {
    // OP_1 OP_IF
    //   OP_0 OP_IF
    //     OP_0 OP_IF OP_1 OP_ENDIF   <- skipped (outer false)
    //   OP_ELSE
    //     OP_1                        <- executed (else branch of level 2)
    //   OP_ENDIF
    // OP_ENDIF
    let tokens = vec![
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpEndIf),
        Token::Op(Opcode::OpElse),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpEndIf),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(run(&tokens).unwrap());
}

#[test]
fn false_outer_skips_inner() {
    // OP_0 OP_IF
    //   OP_1 OP_IF OP_1 OP_ENDIF   <- entire block skipped
    // OP_ENDIF
    // -> empty stack -> false
    let tokens = vec![
        Token::Op(Opcode::Op0),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpIf),
        Token::Op(Opcode::Op1),
        Token::Op(Opcode::OpEndIf),
        Token::Op(Opcode::OpEndIf),
    ];
    assert!(!run(&tokens).unwrap());
}
