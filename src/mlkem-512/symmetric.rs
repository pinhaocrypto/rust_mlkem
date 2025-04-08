
use crate::mlkem_512::params::SYMBYTES;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Digest, 
    Sha3_256,
    Sha3_512,
    Shake256,
};

pub const H_DIGEST_SIZE: usize = SYMBYTES;
pub const G_DIGEST_SIZE: usize = 2 * SYMBYTES;

pub struct MLKEMHashes;

impl MLKEMHashes {
    /// H: SHA3-256(s) -> 32 bytes
    ///
    /// Conforms to FIPS 203, Section 4.4.
    pub fn h(input: &[u8]) -> [u8; H_DIGEST_SIZE] {
        Sha3_256::digest(input).into()
    }

    /// G: SHA3-512(c) -> (32 bytes, 32 bytes)
    ///
    /// Conforms to FIPS 203, Section below 4.4.
    /// This function computes SHA3-512 and splits the 64-byte result.
    /// Splits the 64-byte digest into two 32-byte parts.
    pub fn g(input: &[u8]) -> ([u8; SYMBYTES], [u8; SYMBYTES]) {
        let digest: [u8; G_DIGEST_SIZE] = Sha3_512::digest(input).into();
        let (part1, part2) = digest.split_at(SYMBYTES);
        (
            part1.try_into().expect("Split size should be correct"),
            part2.try_into().expect("Split size should be correct"),
        )
    }

    /// J: SHAKE256(s, 256) -> 32 bytes
    ///
    /// Conforms to FIPS 203, Section 4.4.
    /// Output length for SHAKE256 is specified as 8 * 32 = 256 bits.
    pub fn j(input: &[u8]) -> [u8; SYMBYTES] {
        let mut output = [0u8; SYMBYTES];
        let mut hasher = Shake256::default();
        hasher.update(input);
        let mut reader = hasher.finalize_xof(); // Get an XOF reader
        reader.read(&mut output); // Read exactly SYMBYTES bytes
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
}
    