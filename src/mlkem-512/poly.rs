use crate::mlkem512::params::N;

/// Represents a polynomial in the ring R_q = Z_q[X]/(X^n + 1)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    /// Coefficients of the polynomial
    pub coeffs: [i16; N],
}