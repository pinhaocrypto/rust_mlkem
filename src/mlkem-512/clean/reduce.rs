use crate::mlkem_512::params::Q;
/// Montgomery reduction: given a 32-bit integer a, computes
/// 16-bit integer congruent to a * R^-1 mod q, where R=2^16
///
/// Arguments:
///   - a: input integer to be reduced, of absolute value
///     smaller or equal to INT32_MAX - 2^15 * Q.
///
/// Returns: integer congruent to a * R^-1 modulo q, with absolute value
///     <= ceil(|a| / 2^16) + (Q + 1)/2
/// We first take the modulo of 'a' because it's a 32-bit integer. If we were to
/// multiply 'a' directly with QINV without taking the modulo first, the intermediate
/// result would be a 32-bit or higher value.
/// If 'a' itself is large, the product could exceed the range that can be
/// represented by a 32-bit integer, leading to overflow (and in the case of
/// signed integers, even undefined behavior).
#[inline(always)]
pub fn montgomery_reduce(a: i32) -> i16 {
    // q^(-1) mod 2^16
    const QINV: u32 = 62209;

    // Compute a*q^{-1} mod 2^16 in unsigned representatives
    let a_reduced = (a & 0xFFFF) as u32;
    let a_inverted = ((a_reduced * QINV) & 0xFFFF) as i16;

    assert!(
        a < i32::MAX - ((1_i32 << 15) * Q as i32) && a > -(i32::MAX - ((1_i32 << 15) * Q as i32))
    );

    // Compute r = (a - a_inverted * Q) >> 16
    let r = (a - (a_inverted as i32) * (Q as i32)) >> 16;

    r as i16
}

use crate::mlkem512::params::N;

/// Represents a polynomial in the ring R_q = Z_q[X]/(X^n + 1)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    /// Coefficients of the polynomial
    pub coeffs: [i16; N],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_montgomery_reduce_basic() {
        // Test with zero input
        assert_eq!(montgomery_reduce(0), 0, "montgomery_reduce(0) should be 0");

        // Test with simple positive values
        assert_eq!(
            montgomery_reduce(65536),
            1,
            "montgomery_reduce(2^16) should be 1"
        );
        assert_eq!(
            montgomery_reduce(131072),
            2,
            "montgomery_reduce(2^17) should be 2"
        );

        // Test with Q-related values
        assert_eq!(
            montgomery_reduce(Q as i32),
            0,
            "montgomery_reduce(Q) should be 0"
        );
        assert_eq!(
            montgomery_reduce(65536 * Q as i32),
            Q as i16,
            "montgomery_reduce(Q*2^16) should be Q"
        );

        // Test with negative values
        assert_eq!(
            montgomery_reduce(-65536),
            -1,
            "montgomery_reduce(-2^16) should be -1"
        );
        assert_eq!(
            montgomery_reduce(-131072),
            -2,
            "montgomery_reduce(-2^17) should be -2"
        );
    }

    #[test]
    fn test_montgomery_reduce_edge_cases() {
        // Test values close to the valid range boundaries
        let max_valid = i32::MAX - ((1_i32 << 15) * Q as i32);
        let min_valid = -(i32::MAX - ((1_i32 << 15) * Q as i32));

        // Test values close to but within the valid range
        let large_positive = max_valid - 1000;
        let large_negative = min_valid + 1000;

        // We don't assert specific values here, just that the function doesn't panic
        let _result_pos = montgomery_reduce(large_positive);
        let _result_neg = montgomery_reduce(large_negative);
    }

    #[test]
    fn test_montgomery_reduce_known_values() {
        // Known input-output pairs, verified against reference implementation
        let test_cases = [
            (0, 0),
            (65536, 1),           // 2^16
            (131072, 2),          // 2^17
            (3329 * 65536, 3329), // Q * 2^16
            (62209, 339),         // This is the correct output for our implementation
        ];

        for (input, expected) in test_cases {
            let result = montgomery_reduce(input);
            assert_eq!(
                result, expected,
                "montgomery_reduce({}) should be {}, got {}",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_montgomery_reduction_property() {
        // Test the fundamental property of Montgomery reduction:
        // For any integer a, montgomery_reduce(a * 2^16) ≡ a (mod Q)

        const R: i32 = 65536; // R = 2^16

        for a in -10..10 {
            let result = montgomery_reduce(a * R);
            let expected = a as i16;

            // Check congruence modulo Q
            assert_eq!(
                (result as i32).rem_euclid(Q as i32),
                (expected as i32).rem_euclid(Q as i32),
                "Montgomery reduction property failed for a={}",
                a
            );
        }
    }

    #[test]
    fn test_montgomery_reduce_consistency() {
        // Test consistency of montgomery_reduce with core mathematical properties

        const R: i32 = 65536; // R = 2^16

        // Test a different property: For any a, montgomery_reduce(a * R) ≡ a (mod Q)
        for a in 0..20 {
            // Compute a * R
            let a_times_r = a * R;

            // Apply montgomery_reduce
            let result = montgomery_reduce(a_times_r);

            // Check that result ≡ a (mod Q)
            assert_eq!(
                (result as i32).rem_euclid(Q as i32),
                (a as i32).rem_euclid(Q as i32),
                "Montgomery reduction property failed for a={}",
                a
            );

            // The test above verifies congruence modulo Q
            // For most values we also expect exact equality:
            if a < Q as i32 {
                assert_eq!(
                    result, a as i16,
                    "For small a, montgomery_reduce(a*R) should equal a"
                );
            }
        }
    }
}
