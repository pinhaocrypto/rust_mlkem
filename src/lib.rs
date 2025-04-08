/*
 * Copyright (c) 2024-2025 The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! ML-KEM: Module-Lattice based Key Encapsulation Mechanism
//! Post-quantum cryptographic algorithm standardized by NIST

#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

// Reference the ML-KEM-512 implementation using the path attribute
// #[path = "mlkem-512/clean/mod.rs"]
#[path = "mlkem-512/mod.rs"]
pub mod mlkem_512;

// Re-export primary API from mlkem_512
// pub use mlkem_512::{keypair, encapsulate, decapsulate};

// Provide a more convenient alias
pub mod mlkem512 {
    pub use crate::mlkem_512::*;
}

// todo
// Public constants for ML-KEM variants
// pub const MLKEM512_PUBLIC_KEY_BYTES: usize = crate::mlkem_512::PUBLIC_KEY_BYTES;
// pub const MLKEM512_SECRET_KEY_BYTES: usize = crate::mlkem_512::SECRET_KEY_BYTES;
// pub const MLKEM512_CIPHERTEXT_BYTES: usize = crate::mlkem_512::CIPHERTEXT_BYTES;
// pub const MLKEM512_SHARED_SECRET_BYTES: usize = crate::mlkem_512::SHARED_SECRET_BYTES;

pub const MLKEM768_PUBLIC_KEY_BYTES: usize = 1184;
pub const MLKEM768_SECRET_KEY_BYTES: usize = 2400;
pub const MLKEM768_CIPHERTEXT_BYTES: usize = 1088;
pub const MLKEM768_SHARED_SECRET_BYTES: usize = 32;

pub const MLKEM1024_PUBLIC_KEY_BYTES: usize = 1568;
pub const MLKEM1024_SECRET_KEY_BYTES: usize = 3168;
pub const MLKEM1024_CIPHERTEXT_BYTES: usize = 1568;
pub const MLKEM1024_SHARED_SECRET_BYTES: usize = 32;
