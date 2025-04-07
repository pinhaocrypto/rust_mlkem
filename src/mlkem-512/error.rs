use thiserror::Error;

/// Represents errors that can occur during ML-KEM operations.
#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum KemError {
    /// Input data is invalid (e.g., incorrect length).
    #[error("Invalid input provided.")]
    InvalidInput,

    /// The provided public or secret key is invalid.
    #[error("Invalid key format or parameters.")]
    InvalidKey,

    /// Decapsulation failed, likely due to an invalid ciphertext
    /// or a mismatch between keys. The produced shared secret is invalid.
    #[error("Decapsulation failed. Ciphertext may be invalid.")]
    DecapsulationFailure,

    /// An error occurred during the generation of random bytes.
    #[error("Failed to generate sufficient randomness.")]
    RandomnessError,

    /// An unexpected internal error occurred.
    #[error("An internal cryptographic error occurred.")]
    InternalError,
}
