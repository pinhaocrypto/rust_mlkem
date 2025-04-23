use crate::mlkem512::params::N;
use crate::mlkem512::poly::Poly;
use crate::mlkem512::zetas::ZETAS;
/// Performs a block of Cooley–Tukey NTT butterfly operations on polynomial coefficients in-place,
/// using a fixed twiddle factor and Montgomery multiplication.
///
/// # Parameters
///
/// * `zeta`: Twiddle factor in Montgomery form (signed canonical).
/// * `start`: Starting index of the butterfly block.
/// * `len`: Half the block size; distance between paired coefficients.
///
/// # Behavior
///
/// For each `j` in `start..start + len`, computes:
/// ```ignore
/// let t = crate::mlkem512::backend::reduce::fqmul(self.coeffs[j + len], zeta);
/// self.coeffs[j + len] = self.coeffs[j] - t;
/// self.coeffs[j]       = self.coeffs[j] + t;
/// ```
///
/// Coefficients in `[start, start + 2*len)` may temporarily exceed the usual bounds by `MLKEM_Q`.
///
/// # Examples
///
/// ```text
/// // start = 8, len = 4 -> pairs: (8, 12), (9, 13), (10, 14), (11, 15)
/// // start = 4, len = 2 -> pairs: (4, 6), (5, 7)
/// ```


#[inline]
pub fn ntt_butterfly_block(
    poly: &mut Poly,
    zeta: i16,
    start: usize,
    len: usize,
) {
    for j in start..start + len {
        let t: i16 = crate::mlkem512::backend::reduce::fqmul(poly.coeffs[j + len], zeta);
        poly.coeffs[j + len] = poly.coeffs[j] - t;
        poly.coeffs[j]       = poly.coeffs[j] + t;
    }
}

#[inline]
pub fn ntt_layer(
    poly: &mut Poly,
    layer: usize,
) {
    let len = N >> layer;
    let mut k = 1 << (layer-1);
    for start in (0..N).step_by(2 * len) {
        let zeta = ZETAS[k];
        k += 1;
        ntt_butterfly_block(poly, zeta, start, len);
    }
}