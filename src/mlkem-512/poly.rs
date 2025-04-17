use crate::mlkem512::params::N;


/// Represents a polynomial in the ring R_q = Z_q[X]/(X^n + 1)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    /// Coefficients of the polynomial
    pub coeffs: [i16; N],
}

// Default implementation that dispatches to the appropriate optimized version

impl Poly {
    pub fn new() -> Self {
        Self { coeffs: [0; N] }
    }

    pub fn poly_tomont(&mut self) {
        #[cfg(feature = "clean")]
        {
            crate::mlkem512::clean::poly::poly_tomont(self);
        }
        #[cfg(feature = "avx2")]
        {
            crate::mlkem512::avx2::poly::to_mont(self);
        }
        #[cfg(feature = "avx512")]
        {
            crate::mlkem512::avx512::poly::to_mont(self);
        }
        #[cfg(feature = "neon")]
        {
            crate::mlkem512::neon::poly::to_mont(self);
        }
    }
}


// Elements of R_q = Z_q[X]/(X^n + 1). Represents polynomial
// coeffs[0] + X*coeffs[1] + X^2*coeffs[2] + ... + X^{n-1}*coeffs[n-1]
