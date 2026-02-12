/// A Bitcoin Script opcode supported by this engine.
///
/// This is a fieldless enum that maps 1:1 to protocol-defined byte values.
/// It derives `Copy` because it carries no heap data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Constants
    Op0,
    Op1Negate,
    Op1,
    Op2,
    Op3,
    Op4,
    Op5,
    Op6,
    Op7,
    Op8,
    Op9,
    Op10,
    Op11,
    Op12,
    Op13,
    Op14,
    Op15,
    Op16,

    // Flow control
    OpNop,
    OpIf,
    OpNotIf,
    OpElse,
    OpEndIf,
    OpVerify,
    OpReturn,

    // Stack manipulation
    Op2Drop,
    Op2Dup,
    OpDepth,
    OpDrop,
    OpDup,
    OpNip,
    OpOver,
    OpSwap,
    OpTuck,

    // Splice
    OpSize,

    // Comparison
    OpEqual,
    OpEqualVerify,

    // Logic
    OpNot,

    // Crypto
    OpRipemd160,
    OpSha256,
    OpHash160,
    OpHash256,
    OpCheckSig,
    OpCheckSigVerify,
}

impl Opcode {
    /// Convert a byte to an `Opcode`, if it maps to a supported opcode.
    ///
    /// Returns `None` for push-data bytes (`0x01`-`0x4e`), reserved opcodes,
    /// and any unimplemented opcode. These are handled by the tokenizer
    /// (push-data) or rejected as unsupported.
    pub fn from_byte(byte: u8) -> Option<Opcode> {
        match byte {
            0x00 => Some(Opcode::Op0),
            0x4f => Some(Opcode::Op1Negate),
            0x51 => Some(Opcode::Op1),
            0x52 => Some(Opcode::Op2),
            0x53 => Some(Opcode::Op3),
            0x54 => Some(Opcode::Op4),
            0x55 => Some(Opcode::Op5),
            0x56 => Some(Opcode::Op6),
            0x57 => Some(Opcode::Op7),
            0x58 => Some(Opcode::Op8),
            0x59 => Some(Opcode::Op9),
            0x5a => Some(Opcode::Op10),
            0x5b => Some(Opcode::Op11),
            0x5c => Some(Opcode::Op12),
            0x5d => Some(Opcode::Op13),
            0x5e => Some(Opcode::Op14),
            0x5f => Some(Opcode::Op15),
            0x60 => Some(Opcode::Op16),
            0x61 => Some(Opcode::OpNop),
            0x63 => Some(Opcode::OpIf),
            0x64 => Some(Opcode::OpNotIf),
            0x67 => Some(Opcode::OpElse),
            0x68 => Some(Opcode::OpEndIf),
            0x69 => Some(Opcode::OpVerify),
            0x6a => Some(Opcode::OpReturn),
            0x6d => Some(Opcode::Op2Drop),
            0x6e => Some(Opcode::Op2Dup),
            0x74 => Some(Opcode::OpDepth),
            0x75 => Some(Opcode::OpDrop),
            0x76 => Some(Opcode::OpDup),
            0x77 => Some(Opcode::OpNip),
            0x78 => Some(Opcode::OpOver),
            0x7c => Some(Opcode::OpSwap),
            0x7d => Some(Opcode::OpTuck),
            0x82 => Some(Opcode::OpSize),
            0x87 => Some(Opcode::OpEqual),
            0x88 => Some(Opcode::OpEqualVerify),
            0x91 => Some(Opcode::OpNot),
            0xa6 => Some(Opcode::OpRipemd160),
            0xa8 => Some(Opcode::OpSha256),
            0xa9 => Some(Opcode::OpHash160),
            0xaa => Some(Opcode::OpHash256),
            0xac => Some(Opcode::OpCheckSig),
            0xad => Some(Opcode::OpCheckSigVerify),
            _ => None,
        }
    }

    /// Convert an `Opcode` back to its canonical byte value.
    pub fn to_byte(self) -> u8 {
        match self {
            Opcode::Op0 => 0x00,
            Opcode::Op1Negate => 0x4f,
            Opcode::Op1 => 0x51,
            Opcode::Op2 => 0x52,
            Opcode::Op3 => 0x53,
            Opcode::Op4 => 0x54,
            Opcode::Op5 => 0x55,
            Opcode::Op6 => 0x56,
            Opcode::Op7 => 0x57,
            Opcode::Op8 => 0x58,
            Opcode::Op9 => 0x59,
            Opcode::Op10 => 0x5a,
            Opcode::Op11 => 0x5b,
            Opcode::Op12 => 0x5c,
            Opcode::Op13 => 0x5d,
            Opcode::Op14 => 0x5e,
            Opcode::Op15 => 0x5f,
            Opcode::Op16 => 0x60,
            Opcode::OpNop => 0x61,
            Opcode::OpIf => 0x63,
            Opcode::OpNotIf => 0x64,
            Opcode::OpElse => 0x67,
            Opcode::OpEndIf => 0x68,
            Opcode::OpVerify => 0x69,
            Opcode::OpReturn => 0x6a,
            Opcode::Op2Drop => 0x6d,
            Opcode::Op2Dup => 0x6e,
            Opcode::OpDepth => 0x74,
            Opcode::OpDrop => 0x75,
            Opcode::OpDup => 0x76,
            Opcode::OpNip => 0x77,
            Opcode::OpOver => 0x78,
            Opcode::OpSwap => 0x7c,
            Opcode::OpTuck => 0x7d,
            Opcode::OpSize => 0x82,
            Opcode::OpEqual => 0x87,
            Opcode::OpEqualVerify => 0x88,
            Opcode::OpNot => 0x91,
            Opcode::OpRipemd160 => 0xa6,
            Opcode::OpSha256 => 0xa8,
            Opcode::OpHash160 => 0xa9,
            Opcode::OpHash256 => 0xaa,
            Opcode::OpCheckSig => 0xac,
            Opcode::OpCheckSigVerify => 0xad,
        }
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Opcode::Op0 => "OP_0",
            Opcode::Op1Negate => "OP_1NEGATE",
            Opcode::Op1 => "OP_1",
            Opcode::Op2 => "OP_2",
            Opcode::Op3 => "OP_3",
            Opcode::Op4 => "OP_4",
            Opcode::Op5 => "OP_5",
            Opcode::Op6 => "OP_6",
            Opcode::Op7 => "OP_7",
            Opcode::Op8 => "OP_8",
            Opcode::Op9 => "OP_9",
            Opcode::Op10 => "OP_10",
            Opcode::Op11 => "OP_11",
            Opcode::Op12 => "OP_12",
            Opcode::Op13 => "OP_13",
            Opcode::Op14 => "OP_14",
            Opcode::Op15 => "OP_15",
            Opcode::Op16 => "OP_16",
            Opcode::OpNop => "OP_NOP",
            Opcode::OpIf => "OP_IF",
            Opcode::OpNotIf => "OP_NOTIF",
            Opcode::OpElse => "OP_ELSE",
            Opcode::OpEndIf => "OP_ENDIF",
            Opcode::OpVerify => "OP_VERIFY",
            Opcode::OpReturn => "OP_RETURN",
            Opcode::Op2Drop => "OP_2DROP",
            Opcode::Op2Dup => "OP_2DUP",
            Opcode::OpDepth => "OP_DEPTH",
            Opcode::OpDrop => "OP_DROP",
            Opcode::OpDup => "OP_DUP",
            Opcode::OpNip => "OP_NIP",
            Opcode::OpOver => "OP_OVER",
            Opcode::OpSwap => "OP_SWAP",
            Opcode::OpTuck => "OP_TUCK",
            Opcode::OpSize => "OP_SIZE",
            Opcode::OpEqual => "OP_EQUAL",
            Opcode::OpEqualVerify => "OP_EQUALVERIFY",
            Opcode::OpNot => "OP_NOT",
            Opcode::OpRipemd160 => "OP_RIPEMD160",
            Opcode::OpSha256 => "OP_SHA256",
            Opcode::OpHash160 => "OP_HASH160",
            Opcode::OpHash256 => "OP_HASH256",
            Opcode::OpCheckSig => "OP_CHECKSIG",
            Opcode::OpCheckSigVerify => "OP_CHECKSIGVERIFY",
        };
        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_all_opcodes() {
        let opcodes = [
            Opcode::Op0,
            Opcode::Op1Negate,
            Opcode::Op1,
            Opcode::Op2,
            Opcode::Op3,
            Opcode::Op4,
            Opcode::Op5,
            Opcode::Op6,
            Opcode::Op7,
            Opcode::Op8,
            Opcode::Op9,
            Opcode::Op10,
            Opcode::Op11,
            Opcode::Op12,
            Opcode::Op13,
            Opcode::Op14,
            Opcode::Op15,
            Opcode::Op16,
            Opcode::OpNop,
            Opcode::OpIf,
            Opcode::OpNotIf,
            Opcode::OpElse,
            Opcode::OpEndIf,
            Opcode::OpVerify,
            Opcode::OpReturn,
            Opcode::Op2Drop,
            Opcode::Op2Dup,
            Opcode::OpDepth,
            Opcode::OpDrop,
            Opcode::OpDup,
            Opcode::OpNip,
            Opcode::OpOver,
            Opcode::OpSwap,
            Opcode::OpTuck,
            Opcode::OpSize,
            Opcode::OpEqual,
            Opcode::OpEqualVerify,
            Opcode::OpNot,
            Opcode::OpRipemd160,
            Opcode::OpSha256,
            Opcode::OpHash160,
            Opcode::OpHash256,
            Opcode::OpCheckSig,
            Opcode::OpCheckSigVerify,
        ];

        for opcode in &opcodes {
            let byte = opcode.to_byte();
            let recovered = Opcode::from_byte(byte);
            assert_eq!(recovered, Some(*opcode), "roundtrip failed for {opcode}");
        }
    }

    #[test]
    fn push_data_bytes_return_none() {
        for byte in 0x01..=0x4bu8 {
            assert_eq!(
                Opcode::from_byte(byte),
                None,
                "byte 0x{byte:02x} should be None"
            );
        }
        // OP_PUSHDATA1, OP_PUSHDATA2, OP_PUSHDATA4
        assert_eq!(Opcode::from_byte(0x4c), None);
        assert_eq!(Opcode::from_byte(0x4d), None);
        assert_eq!(Opcode::from_byte(0x4e), None);
    }

    #[test]
    fn unsupported_bytes_return_none() {
        assert_eq!(Opcode::from_byte(0x50), None); // OP_RESERVED
        assert_eq!(Opcode::from_byte(0xb0), None);
        assert_eq!(Opcode::from_byte(0xff), None);
    }

    #[test]
    fn display_formatting() {
        assert_eq!(format!("{}", Opcode::OpDup), "OP_DUP");
        assert_eq!(format!("{}", Opcode::OpHash160), "OP_HASH160");
        assert_eq!(format!("{}", Opcode::Op0), "OP_0");
        assert_eq!(format!("{}", Opcode::OpCheckSig), "OP_CHECKSIG");
    }
}
