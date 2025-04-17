use crate::mlkem512::poly::Poly;
use crate::mlkem512::clean::reduce::fqmul;

// Implementation-specific functions
pub fn poly_tomont(r: &mut Poly) {
    const F: i16 = 1353; // check-magic: 1353 == signed_mod(2^32, Q)
    
    r.coeffs.iter_mut().for_each(|coeff| {
        *coeff = fqmul(*coeff, F);
    });
}

// Other clean implementations
// pub fn ntt(r: &mut Poly) { ... }
// pub fn inv_ntt(r: &mut Poly) { ... }