use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

/// Computes the SHA-256 hash of the input data.
///
/// Returns a 32-byte digest. This is the fundamental hash primitive
/// used throughout Bitcoin for transaction hashing, block hashing,
/// and as a building block for [`hash160`] and [`hash256`].
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes the RIPEMD-160 hash of the input data.
///
/// Returns a 20-byte digest. Used in Bitcoin as the second step
/// of the [`hash160`] address-derivation hash.
pub fn ripemd160(data: &[u8]) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Computes HASH160: RIPEMD-160 of SHA-256.
///
/// This is the standard Bitcoin address hash function, used in P2PKH
/// to derive a 20-byte address from a public key.
///
/// `hash160(data) = ripemd160(sha256(data))`
pub fn hash160(data: &[u8]) -> [u8; 20] {
    ripemd160(&sha256(data))
}

/// Computes HASH256: double SHA-256.
///
/// This is the standard Bitcoin transaction and block hash function.
///
/// `hash256(data) = sha256(sha256(data))`
pub fn hash256(data: &[u8]) -> [u8; 32] {
    sha256(&sha256(data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn sha256_empty() {
        let result = sha256(b"");
        assert_eq!(
            result,
            hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
    }

    #[test]
    fn ripemd160_empty() {
        let result = ripemd160(b"");
        assert_eq!(result, hex!("9c1185a5c5e9fc54612808977ee8f548b2258d31"));
    }

    #[test]
    fn hash160_empty() {
        let result = hash160(b"");
        assert_eq!(result, hex!("b472a266d0bd89c13706a4132ccfb16f7c3b9fcb"));
    }

    #[test]
    fn hash256_empty() {
        let result = hash256(b"");
        assert_eq!(
            result,
            hex!("5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456")
        );
    }

    #[test]
    fn sha256_hello() {
        // SHA-256("hello") — well-known test vector
        let result = sha256(b"hello");
        assert_eq!(
            result,
            hex!("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
        );
    }

    #[test]
    fn hash160_is_ripemd160_of_sha256() {
        let data = b"test composition";
        let expected = ripemd160(&sha256(data));
        assert_eq!(hash160(data), expected);
    }

    #[test]
    fn hash256_is_double_sha256() {
        let data = b"test composition";
        let expected = sha256(&sha256(data));
        assert_eq!(hash256(data), expected);
    }
}
