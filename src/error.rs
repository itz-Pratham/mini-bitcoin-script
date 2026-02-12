/// All error conditions that can arise during script parsing or execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    /// Stack had fewer elements than the operation required.
    StackUnderflow,

    /// Script byte stream ended mid-instruction.
    UnexpectedEndOfScript,

    /// A push-data length field is malformed or exceeds remaining bytes.
    InvalidPushData,

    /// An opcode byte is valid in Bitcoin but not implemented by this engine.
    UnsupportedOpcode(u8),

    /// OP_VERIFY, OP_EQUALVERIFY, or OP_CHECKSIGVERIFY consumed a false value.
    VerifyFailed,

    /// Execution completed but the stack is empty or the top element is false.
    ScriptFailed,

    /// OP_RETURN was encountered. The script is provably unspendable.
    OpReturnEncountered,

    /// OP_IF / OP_NOTIF / OP_ELSE / OP_ENDIF are not properly balanced.
    UnbalancedConditional,

    /// A hex string could not be decoded (odd length or invalid character).
    InvalidHex,
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptError::StackUnderflow => {
                write!(f, "stack underflow: not enough elements on the stack")
            }
            ScriptError::UnexpectedEndOfScript => {
                write!(f, "unexpected end of script")
            }
            ScriptError::InvalidPushData => {
                write!(f, "invalid push data encoding")
            }
            ScriptError::UnsupportedOpcode(b) => {
                write!(f, "unsupported opcode: 0x{b:02x}")
            }
            ScriptError::VerifyFailed => {
                write!(f, "verify failed: top stack element is false")
            }
            ScriptError::ScriptFailed => {
                write!(f, "script failed: final stack state is false")
            }
            ScriptError::OpReturnEncountered => {
                write!(f, "OP_RETURN encountered: script is unspendable")
            }
            ScriptError::UnbalancedConditional => {
                write!(f, "unbalanced conditional: mismatched IF/ELSE/ENDIF")
            }
            ScriptError::InvalidHex => {
                write!(f, "invalid hex string")
            }
        }
    }
}

impl std::error::Error for ScriptError {}
