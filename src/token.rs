use crate::opcode::Opcode;

/// A parsed script element — either an opcode instruction or pushed data.
///
/// This is the output of the tokenizer and the input to the execution engine.
/// `PushData` carries the raw bytes from any of the four push-data encodings.
/// `Op` wraps a fieldless [`Opcode`] variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// Data pushed onto the stack by a push-data instruction.
    /// Covers: direct push (0x01-0x4b), OP_PUSHDATA1, OP_PUSHDATA2, OP_PUSHDATA4.
    PushData(Vec<u8>),

    /// An opcode instruction (any non-push operation).
    Op(Opcode),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::PushData(data) => {
                write!(f, "<")?;
                for byte in data {
                    write!(f, "{byte:02x}")?;
                }
                write!(f, ">")
            }
            Token::Op(opcode) => write!(f, "{opcode}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_push_data() {
        let token = Token::PushData(vec![0x89, 0xab, 0xcd, 0xef]);
        assert_eq!(format!("{token}"), "<89abcdef>");
    }

    #[test]
    fn display_empty_push_data() {
        let token = Token::PushData(vec![]);
        assert_eq!(format!("{token}"), "<>");
    }

    #[test]
    fn display_opcode() {
        let token = Token::Op(Opcode::OpDup);
        assert_eq!(format!("{token}"), "OP_DUP");
    }
}
