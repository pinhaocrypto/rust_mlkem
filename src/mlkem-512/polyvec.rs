// mlk_polyvec_reduce(mlk_polyvec *r)
// mlk_polyvec_add(mlk_polyvec *r, const mlk_polyvec *b)
// mlk_polyvec_tomont(mlk_polyvec *r)
// mlk_polyvec_ntt(mlk_polyvec *r)
// mlk_polyvec_invntt_tomont(mlk_polyvec *r)
// mlk_polyvec_basemul_acc_montgomery(mlk_poly *r, const mlk_polyvec *a, const mlk_polyvec *b)
// mlk_polyvec_pointwise_acc_montgomery(mlk_poly *r, const mlk_polyvec *a, const mlk_polyvec *b)
// mlk_polyvec_frombytes(mlk_polyvec *r, const uint8_t *a)
// mlk_polyvec_tobytes(uint8_t *r, const mlk_polyvec *a)
// mlk_polyvec_compress(uint8_t *r, const mlk_polyvec *a)
// mlk_polyvec_decompress(mlk_polyvec *r, const uint8_t *a)
// mlk_polyvec_csubq(mlk_polyvec *r)
// mlk_polyvec_zeroize(mlk_polyvec *r)
// mlk_polyvec_copy(mlk_polyvec *dest, const mlk_polyvec *src)
// mlk_polyvec_shift(mlk_polyvec *r, int shift)

use crate::mlkem512::params::K;
use crate::mlkem512::poly::Poly;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolyVec {
    /// Coefficients of the polynomial
    pub vec: [Poly; K],
}

impl PolyVec {
    // pub fn new() -> Self {
    //     Self { vec: [Poly::new(); K] }
    // }
    /*************************************************
    * Name:        polyvec_compress
    *
    * Description: Compress and serialize vector of polynomials
    *
    * Arguments:   - uint8_t *r: pointer to output byte array
    *                            (needs space for KYBER_POLYVECCOMPRESSEDBYTES)
    *              - const polyvec *a: pointer to input vector of polynomials
    **************************************************/

    pub fn polyvec_compress() {
        unimplemented!()
    }
}

