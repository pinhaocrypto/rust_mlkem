/*
 * Copyright (c) 2024-2025 The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! ML-KEM-512 implementation: A post-quantum key encapsulation mechanism.

// Module exports
pub mod cbd;
pub mod ntt;
pub mod poly;
pub mod reduce;


// Re-export primary API
// pub use kem::{keypair, encapsulate, decapsulate};

// Public constants for ML-KEM-512
pub const PUBLIC_KEY_BYTES: usize = 800;
pub const SECRET_KEY_BYTES: usize = 1632;
pub const CIPHERTEXT_BYTES: usize = 768;
pub const SHARED_SECRET_BYTES: usize = 32;

pub use crate::mlkem512::backend::clean::poly::poly_tomont;