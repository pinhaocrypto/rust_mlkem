// src/params.rs

// ============================================================================
// Common constants (shared across all parameter sets)
// ============================================================================
pub const MLKEM_N: i16 = 256;
pub const MLKEM_Q: u16 = 3329;
pub const MLKEM_QINV: u16 = 62209; // q^(-1) mod 2^16
pub const MLKEM_Q_HALF: u16 = 1665; 
pub const MLKEM_SYMBYTES: u16 = 32; // size in bytes of hashes, and seeds
pub const MLKEM_SSBYTES: u16 = 32; // size in bytes of shared key
pub const MLKEM_POLYBYTES: u16 = 384;


pub const MLKEM_POLYCOMPRESSEDBYTES_D4: u16 = 128;
pub const MLKEM_POLYCOMPRESSEDBYTES_D5: u16 = 160;
pub const MLKEM_POLYCOMPRESSEDBYTES_D10: u16 = 320;
pub const MLKEM_POLYCOMPRESSEDBYTES_D11: u16 = 352;

// Safety bounds
pub const MLKEM_UINT12_LIMIT: i32 = 4096;


// ============================================================================
// Parameter set specific constants
// ============================================================================

#[cfg(feature = "mlkem512")]
mod mlkem512_params {
    pub const MLKEM_K: u8 = 2;
    pub const MLKEM_ETA1: u8 = 3;
    pub const MLKEM_ETA2: u8 = 2;
    pub const MLKEM_DU: u8 = 10;
    pub const MLKEM_DV: u8 = 4;

    // Compression constants specific to this parameter set
    pub const MLKEM_POLYCOMPRESSEDBYTES_DV: u16 = super::MLKEM_POLYCOMPRESSEDBYTES_D4;
    pub const MLKEM_POLYCOMPRESSEDBYTES_DU: u16 = super::MLKEM_POLYCOMPRESSEDBYTES_D10;

    // Derived sizes
    pub const MLKEM_POLYVECBYTES: u16 = (MLKEM_K as u16) * super::MLKEM_POLYBYTES;
    pub const MLKEM_POLYVECCOMPRESSEDBYTES_DU: u16 = (MLKEM_K as u16) * MLKEM_POLYCOMPRESSEDBYTES_DU;

    // IND-CPA constants
    pub const MLKEM_INDCPA_MSGBYTES: u16 = super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCPA_PUBLICKEYBYTES: u16 = MLKEM_POLYVECBYTES + super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCPA_SECRETKEYBYTES: u16 = MLKEM_POLYVECBYTES;
    pub const MLKEM_INDCPA_BYTES: u16 = MLKEM_POLYVECCOMPRESSEDBYTES_DU + MLKEM_POLYCOMPRESSEDBYTES_DV;

    // IND-CCA2 constants
    pub const MLKEM_INDCCA_PUBLICKEYBYTES: u16 = MLKEM_INDCPA_PUBLICKEYBYTES;
    pub const MLKEM_INDCCA_SECRETKEYBYTES: u16 = MLKEM_INDCPA_SECRETKEYBYTES + MLKEM_INDCPA_PUBLICKEYBYTES + 2 * super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCCA_CIPHERTEXTBYTES: u16 = MLKEM_INDCPA_BYTES;

}

#[cfg(feature = "mlkem768")]
mod mlkem768_params {
    pub const MLKEM_K: u8 = 3;
    pub const MLKEM_ETA1: u8 = 2;
    pub const MLKEM_ETA2: u8 = 2;
    pub const MLKEM_DU: u8 = 10;
    pub const MLKEM_DV: u8 = 4;

    // Compression constants specific to this parameter set
    pub const MLKEM_POLYCOMPRESSEDBYTES_DV: u16 = super::MLKEM_POLYCOMPRESSEDBYTES_D4;
    pub const MLKEM_POLYCOMPRESSEDBYTES_DU: u16 = super::MLKEM_POLYCOMPRESSEDBYTES_D10;

    // Derived sizes
    pub const MLKEM_POLYVECBYTES: u16 = (MLKEM_K as u16) * super::MLKEM_POLYBYTES;
    pub const MLKEM_POLYVECCOMPRESSEDBYTES_DU: u16 = (MLKEM_K as u16) * MLKEM_POLYCOMPRESSEDBYTES_DU;

    // IND-CPA constants
    pub const MLKEM_INDCPA_MSGBYTES: u16 = super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCPA_PUBLICKEYBYTES: u16 = MLKEM_POLYVECBYTES + super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCPA_SECRETKEYBYTES: u16 = MLKEM_POLYVECBYTES;
    pub const MLKEM_INDCPA_BYTES: u16 = MLKEM_POLYVECCOMPRESSEDBYTES_DU + MLKEM_POLYCOMPRESSEDBYTES_DV;

    // IND-CCA2 constants
    pub const MLKEM_INDCCA_PUBLICKEYBYTES: u16 = MLKEM_INDCPA_PUBLICKEYBYTES;
    pub const MLKEM_INDCCA_SECRETKEYBYTES: u16 = MLKEM_INDCPA_SECRETKEYBYTES + MLKEM_INDCPA_PUBLICKEYBYTES + 2 * super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCCA_CIPHERTEXTBYTES: u16 = MLKEM_INDCPA_BYTES;
}

#[cfg(feature = "mlkem1024")]
mod mlkem1024_params {
    pub const MLKEM_K: u8 = 4;
    pub const MLKEM_ETA1: u8 = 2;
    pub const MLKEM_ETA2: u8 = 2;
    pub const MLKEM_DU: u8 = 11;
    pub const MLKEM_DV: u8 = 5;

    // Compression constants specific to this parameter set
    pub const MLKEM_POLYCOMPRESSEDBYTES_DV: u16 = super::MLKEM_POLYCOMPRESSEDBYTES_D4;
    pub const MLKEM_POLYCOMPRESSEDBYTES_DU: u16 = super::MLKEM_POLYCOMPRESSEDBYTES_D10;

    // Derived sizes
    pub const MLKEM_POLYVECBYTES: u16 = (MLKEM_K as u16) * super::MLKEM_POLYBYTES;
    pub const MLKEM_POLYVECCOMPRESSEDBYTES_DU: u16 = (MLKEM_K as u16) * MLKEM_POLYCOMPRESSEDBYTES_DU;

    // IND-CPA constants
    pub const MLKEM_INDCPA_MSGBYTES: u16 = super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCPA_PUBLICKEYBYTES: u16 = MLKEM_POLYVECBYTES + super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCPA_SECRETKEYBYTES: u16 = MLKEM_POLYVECBYTES;
    pub const MLKEM_INDCPA_BYTES: u16 = MLKEM_POLYVECCOMPRESSEDBYTES_DU + MLKEM_POLYCOMPRESSEDBYTES_DV;

    // IND-CCA2 constants
    pub const MLKEM_INDCCA_PUBLICKEYBYTES: u16 = MLKEM_INDCPA_PUBLICKEYBYTES;
    pub const MLKEM_INDCCA_SECRETKEYBYTES: u16 = MLKEM_INDCPA_SECRETKEYBYTES + MLKEM_INDCPA_PUBLICKEYBYTES + 2 * super::MLKEM_SYMBYTES;
    pub const MLKEM_INDCCA_CIPHERTEXTBYTES: u16 = MLKEM_INDCPA_BYTES;
}

// ============================================================================
// Re-export the selected parameter set
// ============================================================================

#[cfg(feature = "mlkem512")]
pub use mlkem512_params::*;

#[cfg(feature = "mlkem768")]
pub use mlkem768_params::*;

#[cfg(feature = "mlkem1024")]
pub use mlkem1024_params::*;

// ============================================================================
// Compile-time validation
// ============================================================================

#[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
compile_error!("Must enable exactly one of: mlkem512, mlkem768, mlkem1024");

#[cfg(all(feature = "mlkem512", feature = "mlkem768"))]
compile_error!("Cannot enable multiple ML-KEM parameter sets simultaneously");

#[cfg(all(feature = "mlkem512", feature = "mlkem1024"))]
compile_error!("Cannot enable multiple ML-KEM parameter sets simultaneously");

#[cfg(all(feature = "mlkem768", feature = "mlkem1024"))]
compile_error!("Cannot enable multiple ML-KEM parameter sets simultaneously");
