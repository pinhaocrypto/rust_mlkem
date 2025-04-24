use crate::mlkem_512::params::{Q, Q_HALF};
/// Barrett reduction: given a 16-bit integer a, computes
/// centered representative congruent to a mod q in {-(q-1)/2,...,(q-1)/2}
///
/// Arguments:
///   - a: input integer to be reduced
///
/// Returns: integer in {-(q-1)/2,...,(q-1)/2} congruent to a modulo q.
#[inline(always)]
pub fn barrett_reduce(a: i16) -> i16 {
    const MAGIC: i32 = 20159; // check-magic: 20159 == round(2^26 / Q)

    // Right-shift on signed integers is arithmetic shift in Rust
    // This means it preserves the sign bit, as required
    let t = (MAGIC * (a as i32) + (1 << 25)) >> 26;

    // We need 32-bit math to evaluate t * Q and the subsequent subtraction
    let res = (a as i32 - (t * Q as i32)) as i16;

    // In production code, we might want to add debug assertions to ensure
    // the result is in the expected range {-(q-1)/2,...,(q-1)/2}
    debug_assert!(res > -(Q_HALF as i16) && res < Q_HALF as i16);

    res
}

/// Montgomery reduction: given a 32-bit integer a, computes
/// 16-bit integer congruent to a * R^-1 mod q, where R=2^16
///
/// Arguments:
///   - a: input integer to be reduced, of absolute value
///     smaller or equal to INT32_MAX - 2^15 * Q.
///
/// Returns: integer congruent to a * R^-1 modulo q, with absolute value
///     <= ceil(|a| / 2^16) + (Q + 1)/2
#[inline(always)]
pub fn montgomery_reduce(a: i32) -> i16 {
    // q^(-1) mod 2^16
    const QINV: u32 = 62209;

    // Compute a*q^{-1} mod 2^16 in unsigned representatives
    let a_reduced = (a & 0xFFFF) as u32;
    let a_inverted = ((a_reduced * QINV) & 0xFFFF) as i16;

    debug_assert!(
        a < i32::MAX - ((1_i32 << 15) * Q as i32) && a > -(i32::MAX - ((1_i32 << 15) * Q as i32))
    );

    // Compute r = (a - a_inverted * Q) >> 16
    let r = (a - (a_inverted as i32) * (Q as i32)) >> 16;

    r as i16
}

/// Montgomery multiplication modulo Q
///
/// Arguments:
///   - a: first factor. Can be any i16 value.
///   - b: second factor. Must be signed canonical (abs value < (Q+1)/2)
///
/// Returns: 16-bit integer congruent to a*b*R^(-1) mod Q, and
/// smaller than Q in absolute value.
#[inline(always)]
pub fn fqmul(a: i16, b: i16) -> i16 {
    debug_assert!(
        b > -(Q_HALF as i16) && b < Q_HALF as i16,
        "b out of bounds: {}",
        b
    );
    let result = montgomery_reduce((a as i32) * (b as i32));
    debug_assert!(
        result > -(Q as i16) && result < Q as i16,
        "result out of bounds: {}",
        result
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_barrett_reduce() {
        // Test specific values
        assert_eq!(barrett_reduce(0), 0);
        assert_eq!(barrett_reduce(Q as i16), 0);
        assert_eq!(barrett_reduce(-Q as i16), 0);
        assert_eq!(barrett_reduce(1), 1);
        assert_eq!(barrett_reduce(-1), -1);
        assert_eq!(barrett_reduce(Q_HALF as i16), -Q_HALF as i16 + 1);
        assert_eq!(barrett_reduce(-(Q_HALF as i16)), Q_HALF as i16 - 1);

        // Test some larger values
        let test_values = [
            100,
            1000,
            3000,
            5000,
            10000,
            20000,
            30000,
            -100,
            -1000,
            -3000,
            -5000,
            -10000,
            -20000,
            -30000,
            i16::MAX,
            i16::MIN,
            i16::MAX / 2,
            i16::MIN / 2,
        ];

        for &a in test_values.iter() {
            // Compute expected result using manual modular reduction
            // First convert to i32 to avoid overflow
            let mut res = (a as i32) % (Q as i32);

            // Adjust to centered representative in {-(q-1)/2, ..., (q-1)/2}
            if res > Q_HALF as i32 {
                res -= Q as i32;
            } else if res < -(Q_HALF as i32) {
                res += Q as i32;
            }

            let expected = res as i16;
            let actual = barrett_reduce(a);

            // Verify the result
            assert_eq!(
                actual, expected,
                "Barrett reduction failed for a={}: expected {}, got {}",
                a, expected, actual
            );

            // Also verify the result is in the expected range
            assert!(
                actual > -(Q_HALF as i16) && actual < Q_HALF as i16,
                "Barrett reduction result out of range: {}",
                actual,
            );
        }
    }

    #[test]
    fn test_fqmul() {
        // random generate i16 values
        let mut rng = rand::rng();
        for _ in 0..100 {
            let a = rng.random_range(i16::MIN..i16::MAX);
            let b = rng.random_range((-(Q_HALF as i16) +1)..Q_HALF as i16);
            let prod = (a as i32) * (b as i32);
            let expected = montgomery_reduce(prod);
            let actual = fqmul(a, b);
            assert_eq!(actual, expected);
        }
    }
}
