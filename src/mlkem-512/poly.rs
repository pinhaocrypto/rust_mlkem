use crate::mlkem512::backend;
use crate::mlkem512::params::{N, Q};

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
        backend::poly_tomont(self);
    }

    pub fn scalar_signed_to_unsigned_q(c: i16) -> u16 {
        // This is currently a branch-based implementation; it should be replaced with a branchless version to prevent side-channel attacks.
        if c < 0 {
            (c + Q as i16) as u16
        } else {
            c as u16
        }
    }

    #[inline]
    pub fn poly_reduce(&mut self) {
        for coeff in &mut self.coeffs {
            let t: i16 = crate::mlkem512::backend::reduce::barrett_reduce(*coeff);
            let u: u16 = Poly::scalar_signed_to_unsigned_q(t);
            *coeff = u as i16;
        }
    }

    fn poly_add(&mut self, other: &Poly) {
        for (a, b) in self.coeffs.iter_mut().zip(other.coeffs.iter()) {
            *a = *a + *b;
        }
    }

    fn poly_sub(&mut self, other: &Poly) {
        for (a, b) in self.coeffs.iter_mut().zip(other.coeffs.iter()) {
            *a = *a - *b;
        }
    }

    // Reference: Does not exist in the reference implementation.
    // - The reference implementation does not use a
    //   multiplication cache ('mulcache'). This is an idea
    //   originally taken from https://ia.cr/2021/986
    //   and used at the C level here.
    fn poly_mulcache_compute() {
        unimplemented!();
    }

    #[inline]
    pub fn ntt_butterfly_block(&mut self, zeta: i16, start: usize, len: usize) {
        backend::ntt::ntt_butterfly_block(self, zeta, start, len);
    }

    // #[inline]
    // pub fn ntt_layer(&mut self, layer: usize) {
    //     backend::ntt::ntt_layer(self, layer);
    // }

    // #[inline]
    // pub fn poly_ntt(&mut self) {
    //     backend::ntt::poly_ntt(self);
    // }

    // #[inline]
    // pub fn poly_invntt_tomont(&mut self) {
    //     backend::ntt::poly_invntt_tomont(self);
    // }

}