//! ML-KEM-512 Public Parameters
//!
//! This module defines the core parameters specifically for the ML-KEM-512 security level.

// --- Parameters ---

/// Dimension of the polynomial ring R_q = Z_q[X]/(X^N + 1). Always 256 for ML-KEM.
pub const N: usize = 256;
/// Modulus Q. Always 3329 for ML-KEM.
pub const Q: usize = 3329;
/// (Q + 1) / 2, used for centered representations or reductions.
pub const Q_HALF: usize = (Q + 1) / 2; // = 1665
/// Limit for 12-bit unsigned integers, relevant for coefficient packing/unpacking.
pub const UINT12_LIMIT: usize = 4096;

/// Size in bytes of hashes, seeds, and implicit rejection values.
pub const SYMBYTES: usize = 32;
/// Size in bytes of the shared secret output by the KEM.
pub const SSBYTES: usize = 32;

/// Size in bytes of a serialized polynomial (N * 12 / 8).
pub const POLYBYTES: usize = 384;

// --- ML-KEM-512 Parameters ---

/// Security level identifier.
pub const LEVEL: usize = 512;
/// Dimension K of vectors and matrices for ML-KEM-512.
pub const K: usize = 2;
/// Noise parameter eta_1 for secret key `s` and error `e1` for ML-KEM-512.
pub const ETA1: usize = 3;
/// Noise parameter eta_2 for secret key `e` and ciphertext error `e2`. Always 2 for ML-KEM.
pub const ETA2: usize = 2;
/// Compression parameter d_u for ciphertext vector `u` for ML-KEM-512.
pub const DU: usize = 10;
/// Compression parameter d_v for ciphertext polynomial `v` for ML-KEM-512.
pub const DV: usize = 4;
/// Size in bytes of a compressed polynomial for `v` (d_v=4 bits per coeff).
pub const POLYCOMPRESSEDBYTES_DV: usize = 128; // N * 4 / 8
/// Size in bytes of a compressed polynomial for `u` vector elements (d_u=10 bits per coeff).
pub const POLYCOMPRESSEDBYTES_DU: usize = 320; // N * 10 / 8

// --- Derived Sizes for ML-KEM-512 ---

/// Size in bytes of a serialized polynomial vector. K * POLYBYTES.
pub const POLYVECBYTES: usize = K * POLYBYTES; // 2 * 384 = 768
/// Size in bytes of a compressed polynomial vector `u`. K * POLYCOMPRESSEDBYTES_DU.
pub const POLYVECCOMPRESSEDBYTES_DU: usize = K * POLYCOMPRESSEDBYTES_DU; // 2 * 320 = 640

/// Size in bytes of the implicit message in the IND-CPA scheme.
pub const INDCPA_MSGBYTES: usize = SYMBYTES; // 32
/// Size in bytes of the IND-CPA public key (serialized vector `t` + seed `rho`).
pub const INDCPA_PUBLICKEYBYTES: usize = POLYVECBYTES + SYMBYTES; // 768 + 32 = 800
/// Size in bytes of the IND-CPA secret key (serialized vector `s`).
pub const INDCPA_SECRETKEYBYTES: usize = POLYVECBYTES; // 768
/// Size in bytes of the IND-CPA ciphertext (compressed vector `u` + compressed poly `v`).
pub const INDCPA_BYTES: usize = POLYVECCOMPRESSEDBYTES_DU + POLYCOMPRESSEDBYTES_DV; // 640 + 128 = 768

/// Size in bytes of the KEM public key.
pub const KEM_PUBLICKEYBYTES: usize = INDCPA_PUBLICKEYBYTES; // 800
/// Size in bytes of the KEM ciphertext.
pub const KEM_CIPHERTEXTBYTES: usize = INDCPA_BYTES; // 768
/// Size in bytes of the KEM secret key.
/// (IND-CPA sk + IND-CPA pk + H(pk) + implicit rejection value `z`).
pub const KEM_SECRETKEYBYTES: usize = INDCPA_SECRETKEYBYTES + INDCPA_PUBLICKEYBYTES + SYMBYTES + SYMBYTES; // 768 + 800 + 32 + 32 = 1632
