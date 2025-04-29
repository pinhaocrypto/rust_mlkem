use crate::mlkem512::params::{N, Q};
use crate::mlkem512::poly::Poly;
use crate::mlkem512::zetas::ZETAS;
use crate::mlkem512::backend::reduce::{barrett_reduce, fqmul};

// Absolute exclusive upper bound for the output of the forward NTT
pub const NTT_BOUND: i32 = 8 * Q as i32;

// Absolute exclusive upper bound for the output of the inverse NTT
pub const INVNTT_BOUND: i32 = 8 * Q as i32;

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
    let mut k = 1 << (layer - 1);

    for start in (0..N).step_by(2 * len) {
        let zeta = ZETAS[k];
        k += 1;
        ntt_butterfly_block(poly, zeta, start, len);
    }
}

#[inline]
pub fn poly_ntt(poly: &mut Poly) {
    debug_assert!(poly.coeffs.iter().all(|&c| (c as i32).abs() < Q as i32));

    for layer in 1..=7 {
        ntt_layer(poly, layer);
    }
    debug_assert!(poly
        .coeffs
        .iter()
        .all(|&c| (c as i32).abs() <= NTT_BOUND as i32));
}

#[inline]
pub fn invntt_layer(
    poly: &mut Poly, 
    layer: usize
) {
    debug_assert!((1..=7).contains(&layer), "invalid invNTT layer");

    let coeffs = &mut poly.coeffs;
    let len = N >> layer;                    
    let mut k = (1 << layer) - 1;            

    for start in (0..N).step_by(2 * len) {
        let zeta = ZETAS[k];
        k -= 1;

        for j in start..start + len {
            let t = coeffs[j];
            let sum = t as i32 + coeffs[j + len] as i32;
            let v = barrett_reduce(sum as i16);
            coeffs[j] = v;

            let diff = coeffs[j + len] - t;
            coeffs[j + len] = fqmul(diff, zeta);
        }
    }

    debug_assert!(
        coeffs.iter().all(|&c| (c as i32).abs() < crate::mlkem512::params::Q as i32),
        "invNTT layer output out of bound"
    );

}

#[inline]
pub fn invntt_tomont(poly: &mut Poly) {

    const F: i16 = 1441;
    for c in &mut poly.coeffs {
        *c = fqmul(*c, F);
    }

    // run invNTT layers from layer=7 down to 1
    for layer in (1..=7).rev() {
        invntt_layer(poly, layer);
    }
    

    // final bound check: |c| < INVNTT_BOUND
    debug_assert!(
        poly.coeffs.iter().all(|&c| (c as i32).abs() < INVNTT_BOUND as i32),
        "invNTT output exceeded bound"
    );
}

#[cfg(test)]
mod test {
    use crate::mlkem512::{backend::poly::poly_reduce, poly_tomont};

    use super::*;
    // src/mlkem512/poly/tests.rs
    use rand::{Rng, SeedableRng};     // 放在 dev-dependencies 裡：rand = { version = "0.8", features = ["std", "rngs"] }
    use rand_chacha::ChaCha20Rng;     // 固定 RNG，讓失敗可重現
    use pretty_assertions::assert_eq; // optional：失敗時用彩色 diff

    /// Generate a random polynomial with coefficients in [-Q/2, Q/2)
    fn random_poly(rng: &mut impl Rng) -> Poly {
        let mut p = Poly { coeffs: [0i16; N] };
        for c in p.coeffs.iter_mut() {
            // uniform random in [-Q/2, Q/2)
            *c = rng.random_range(-(Q as i16) / 2..(Q as i16) / 2);
        }
        p
    }

    #[test]
    fn test_poly_ntt() {
        let mut rng = ChaCha20Rng::seed_from_u64(0x5eed_u64);
        let mut poly = random_poly(&mut rng);
        let expect = poly.clone();  
        println!("poly: {:?}\n", poly.coeffs);
        poly_ntt(&mut poly);
        println!("ntt poly: {:?}", poly.coeffs);

    }

    #[test]
    fn roundtrip_ntt_invntt() {
        let mut rng = ChaCha20Rng::seed_from_u64(0x5eed_u64);
        for i in 0..1000 {
            let mut poly = random_poly(&mut rng);
            let mut expect = poly.clone();          
            poly_ntt(&mut poly);
            invntt_tomont(&mut poly);
            poly_tomont(&mut expect);
            for (&a, &b) in expect.coeffs.iter().zip(poly.coeffs.iter()) {
                assert_eq!((i32::from(a) - i32::from(b)).rem_euclid(Q as i32), 0);
            }
            // assert_eq!(expect, poly, "\n=== NTT round-trip failed ===\n");
            println!("test {} passed", i);
        }
    }
}