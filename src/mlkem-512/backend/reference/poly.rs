use crate::mlkem512::poly::Poly;
use crate::mlkem512::reference::reduce::{barrett_reduce, fqmul};

// Implementation-specific functions
pub fn poly_tomont(r: &mut Poly) {
    const F: i16 = 1353; // check-magic: 1353 == signed_mod(2^32, Q)
    
    r.coeffs.iter_mut().for_each(|coeff| {
        *coeff = fqmul(*coeff, F);
    });
}

pub fn poly_reduce(r: &mut Poly) {
    r.coeffs.iter_mut().for_each(|coeff| {
        *coeff = barrett_reduce(*coeff);
        let u: u16 = Poly::scalar_signed_to_unsigned_q(*coeff);
        *coeff = u as i16;
    });
}