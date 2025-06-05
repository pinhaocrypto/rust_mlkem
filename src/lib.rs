//! # ML-KEM: Module-Lattice based Key Encapsulation Mechanism
//!
//! A Rust implementation of the ML-KEM post-quantum cryptographic algorithm with
//! architecture-specific optimizations.
//!
//! ## Features
//!
//! This crate provides:
//! - **ML-KEM-512, ML-KEM-768, ML-KEM-1024** parameter sets
//! - **Architecture-specific optimizations** (AVX2, NEON)
//! - **Automatic backend selection** at compile time
//! - **Cross-platform compatibility** with fallback to reference implementation
//! - **No-std support** for embedded environments
//!
//! ## Architecture Support
//!
//! | Architecture | Optimized Backend | Auto-Detection |
//! |--------------|-------------------|----------------|
//! | x86_64/x86   | AVX2              | ✅             |
//! | aarch64      | NEON              | ✅             |
//! | Others       | Reference         | ✅             |
//!


#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![deny(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::too_many_arguments)]

// ============================================================================
// Module declarations
// ============================================================================

pub mod backend;
pub mod common;
pub mod mlkem;

