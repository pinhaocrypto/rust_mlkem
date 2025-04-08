/*
 * Copyright (c) 2024-2025 The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! ML-KEM-512 implementation: A post-quantum key encapsulation mechanism.

pub mod params;
pub mod poly;
pub mod polyvec;
pub mod symmetric;
pub mod indcpa;
pub mod kem;

pub mod clean;
// pub mod avx2;
// pub mod avx512;
// pub mod neon;


// pub use clean::{
//     PUBLIC_KEY_BYTES,
//     SECRET_KEY_BYTES,
//     CIPHERTEXT_BYTES,
//     SHARED_SECRET_BYTES,
// };

// pub use kem::{keypair, encapsulate, decapsulate};