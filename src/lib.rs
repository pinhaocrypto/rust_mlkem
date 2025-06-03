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

