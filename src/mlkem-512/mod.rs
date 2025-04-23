/*
 * Copyright (c) 2024-2025 The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0
 */

//! ML-KEM-512 implementation: A post-quantum key encapsulation mechanism.

pub mod params;
pub mod polyvec;
pub mod poly;
pub mod symmetric;
pub mod indcpa;
pub mod kem;
pub mod zetas;

pub mod backend;            // 前面那個 backend/mod.rs

// Re-export public APIs from the backend module (such as poly_reduce, barrett_reduce, etc.)
// so they can be accessed directly from this module without the backend:: prefix.

pub use backend::*;
pub use backend::poly_tomont;
