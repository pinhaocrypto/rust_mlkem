use crate::mlkem_512::params::SYMBYTES;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Digest, 
    Sha3_256,
    Sha3_512,
    Shake256,
};

use super::params::{ETA1, ETA2};

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
    /// PRF function, [FIPS 203, Section 4.1, Eq (4.3)]
    /// Referring to (eq 4.3), `OUT` is assumed to contain `s || b`
    pub fn prf_eta1(input: &[u8]) -> [u8; ETA1] {
        let mut output = [0u8; ETA1];
        let mut hasher = Shake256::default();
        hasher.update(input);
        let mut reader = hasher.finalize_xof();
        reader.read(&mut output);
        output
    }

    pub fn prf_eta2(input: &[u8]) -> [u8; ETA2] {
        let mut output = [0u8; ETA2];
        let mut hasher = Shake256::default();
        hasher.update(input);
        let mut reader = hasher.finalize_xof();
        reader.read(&mut output);
        output
    }

}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_h_function() {
        // Test vector
        let input = b"test input for H function";
        let hash = MLKEMHashes::h(input);
        
        // Verify output size
        assert_eq!(hash.len(), H_DIGEST_SIZE);
        
        // Verify deterministic behavior
        let hash2 = MLKEMHashes::h(input);
        assert_eq!(hash, hash2);
        
        // Verify different inputs produce different outputs
        let different_input = b"different test input";
        let different_hash = MLKEMHashes::h(different_input);
        assert_ne!(hash, different_hash);
    }
    
    #[test]
    fn test_g_function() {
        // Test vector
        let input = b"test input for G function";
        let (k, r) = MLKEMHashes::g(input);
        
        // Verify output sizes
        assert_eq!(k.len(), SYMBYTES);
        assert_eq!(r.len(), SYMBYTES);
        
        // Verify deterministic behavior
        let (k2, r2) = MLKEMHashes::g(input);
        assert_eq!(k, k2);
        assert_eq!(r, r2);
        
        // Verify different inputs produce different outputs
        let different_input = b"different test input";
        let (different_k, different_r) = MLKEMHashes::g(different_input);
        assert!(k != different_k || r != different_r);
    }
    
    #[test]
    fn test_j_function() {
        // Test vector
        let input = b"test input for J function";
        let output = MLKEMHashes::j(input);
        
        // Verify output size
        assert_eq!(output.len(), SYMBYTES);
        
        // Verify deterministic behavior
        let output2 = MLKEMHashes::j(input);
        assert_eq!(output, output2);
        
        // Verify different inputs produce different outputs
        let different_input = b"different test input";
        let different_output = MLKEMHashes::j(different_input);
        assert_ne!(output, different_output);
    }
    
    #[test]
    fn test_prf_eta1() {
        // Test vector
        let input = b"test input for PRF eta1";
        let output = MLKEMHashes::prf_eta1(input);
        
        // Verify output size
        assert_eq!(output.len(), ETA1);
        
        // Verify deterministic behavior
        let output2 = MLKEMHashes::prf_eta1(input);
        assert_eq!(output, output2);
        
        // Verify different inputs produce different outputs
        let different_input = b"different test input";
        let different_output = MLKEMHashes::prf_eta1(different_input);
        assert_ne!(output, different_output);
    }
    
    #[test]
    fn test_prf_eta2() {
        // Test vector
        let input = b"test input for PRF eta2";
        let output = MLKEMHashes::prf_eta2(input);
        
        // Verify output size
        assert_eq!(output.len(), ETA2);
        
        // Verify deterministic behavior
        let output2 = MLKEMHashes::prf_eta2(input);
        assert_eq!(output, output2);
        
        // Verify different inputs produce different outputs
        let different_input = b"different test input";
        let different_output = MLKEMHashes::prf_eta2(different_input);
        assert_ne!(output, different_output);
    }
    
    #[test]
    fn test_known_vectors() {
        // This test would contain known test vectors from the ML-KEM specification
        // For example, from FIPS 203 test vectors if available
        
        // Example (replace with actual test vectors from the specification):
        // let input = hex::decode("00112233445566778899aabbccddeeff").unwrap();
        // let expected_h = hex::decode("...").unwrap();
        // assert_eq!(MLKEMHashes::h(&input), expected_h.as_slice());
    }
}