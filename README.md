# ML-KEM: Module-Lattice based Key Encapsulation Mechanism

[![Rust](https://img.shields.io/badge/rust-2024--edition-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> **⚠️ DEVELOPMENT STATUS: Currently Paused**  
> This project is currently on hold pending further development. The codebase contains foundational implementations but is not yet complete or ready for production use.

A Rust implementation of the **ML-KEM (Module-Lattice based Key Encapsulation Mechanism)** post-quantum cryptographic algorithm, as standardized in [FIPS 203](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf).

## 🚀 Features

- **Multiple Parameter Sets**: Support for ML-KEM-512, ML-KEM-768, and ML-KEM-1024
- **Architecture Optimizations**: Hardware-specific backends for improved performance
- **Cross-Platform**: Compatible with multiple architectures with fallback implementations
- **No-std Support**: Suitable for embedded and resource-constrained environments
- **Memory Safety**: Leverages Rust's ownership system for secure cryptographic operations

## 🏗️ Architecture Support

| Architecture | Optimized Backend | Status         | Auto-Detection |
| ------------ | ----------------- | -------------- | -------------- |
| x86_64/x86   | AVX2              | 🚧 Planned     | ✅             |
| aarch64      | NEON              | 🚧 In Progress | ✅             |
| Others       | Reference         | 🚧 In Progress | ✅             |

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mlkem = "0.1.0"
```

## 🛠️ Features

### Core Features

- `reference` - Reference implementation (enabled by default)
- `native` - Enable architecture-specific optimizations
- `neon` - ARM NEON optimizations (requires `native`)

### Parameter Sets

- `mlkem512` - ML-KEM-512 parameter set (enabled by default)
- `mlkem768` - ML-KEM-768 parameter set
- `mlkem1024` - ML-KEM-1024 parameter set

### Optional Features

- `serde` - Serialization support
- `std` - Standard library support (enabled by default)

## 🚧 Current Implementation Status

### ✅ Completed Components

- **Polynomial Arithmetic**: Core polynomial operations with Montgomery reduction
- **Parameter Definitions**: Constants and configuration for all ML-KEM variants
- **Backend Infrastructure**: Modular backend system for different implementations
- **Memory Layout**: Optimized data structures with proper alignment

### 🚧 In Progress

- **NTT Implementation**: Number Theoretic Transform operations
- **KEM Operations**: Key generation, encapsulation, and decapsulation
- **NEON Backend**: ARM-specific optimizations

### ❌ Not Yet Implemented

- **AVX2 Backend**: x86_64 SIMD optimizations
- **Comprehensive Testing**: Full test suite and benchmarks
- **Documentation**: Complete API documentation
- **Security Auditing**: Constant-time guarantees and side-channel analysis

## 📁 Project Structure

```
src/
├── lib.rs              # Main library entry point
├── backend/            # Architecture-specific implementations
│   ├── mod.rs         # Backend selection logic
│   ├── reference/     # Reference implementation
│   └── neon/          # ARM NEON optimizations
├── common/            # Shared parameters and utilities
│   ├── mod.rs
│   └── params.rs      # ML-KEM parameters and constants
└── mlkem/             # Core ML-KEM implementation
    ├── mod.rs         # Module declarations
    ├── poly.rs        # Polynomial operations
    ├── polyvec.rs     # Polynomial vector operations
    ├── symmetric.rs   # Symmetric primitives
    ├── zetas.rs       # NTT constants
    └── kem.rs         # Key encapsulation mechanism (placeholder)
```

## 🔬 Development

### Building

```bash
# Build with default features
cargo build

# Build with specific parameter set
cargo build --features mlkem768

# Build with NEON optimizations
cargo build --features neon

# Build for no-std environments
cargo build --no-default-features --features mlkem512
```

### Testing

```bash
# Run tests (when implemented)
cargo test

# Run benchmarks (when implemented)
cargo bench
```

## 📚 References

- [FIPS 203: Module-Lattice-Based Key-Encapsulation Mechanism Standard](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf)
- [ML-KEM Native](https://github.com/pq-code-package/mlkem-native)
- [ML-KEM Reference Implementation](https://github.com/pq-crystals/kyber)
- [NIST Post-Quantum Cryptography Standardization](https://csrc.nist.gov/projects/post-quantum-cryptography)

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**PinHao Chen** - [pinhaocrypto@gmail.com](mailto:pinhaocrypto@gmail.com)

---

**Note**: This is an experimental implementation. Do not use in production environments until proper security auditing and testing have been completed.
