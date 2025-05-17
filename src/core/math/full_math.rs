use primitive_types::{U256, U512};
use crate::core::math::types::Q96;

/// Contains 512-bit math functions
/// Facilitates multiplication and division that can have overflow of an intermediate value without any loss of precision
pub struct FullMath;

impl FullMath {
    /// Calculates floor(a×b÷denominator) with full precision
    /// Throws if result overflows a uint256 or denominator == 0
    pub fn mul_div(a: U256, b: U256, denominator: U256) -> Option<U256> {
        if denominator.is_zero() {
            return None;
        }

        // First, multiply a and b
        let mut prod0 = a.full_mul(b);
        let prod1 = prod0 >> 256;
        prod0 = prod0 & U512::MAX;

        // Make sure the result is less than 2^256
        if prod1 >= denominator {
            return None;
        }

        // Handle non-overflow cases, 256 by 256 division
        if prod1 == U256::zero() {
            return Some(prod0 / denominator);
        }

        // Make division exact by subtracting the remainder from [prod1 prod0]
        let remainder = (a * b) % denominator;
        prod1 = prod1 - (if remainder > prod0 { U256::one() } else { U256::zero() });
        prod0 = prod0 - remainder;

        // Factor powers of two out of denominator
        let mut twos = denominator & (!denominator + U256::one()).trailing_zeros();
        denominator = denominator >> twos;

        // Divide [prod1 prod0] by the factors of two
        prod0 = prod0 >> twos;
        prod0 |= prod1 << (256 - twos);
        prod1 = prod1 >> twos;

        // Compute inverse of denominator mod 2^256
        let mut inv = U256::MAX / denominator;
        inv = inv * (U256::from(2) - denominator * inv); // inverse mod 2^8
        inv = inv * (U256::from(2) - denominator * inv); // inverse mod 2^16
        inv = inv * (U256::from(2) - denominator * inv); // inverse mod 2^32
        inv = inv * (U256::from(2) - denominator * inv); // inverse mod 2^64
        inv = inv * (U256::from(2) - denominator * inv); // inverse mod 2^128
        inv = inv * (U256::from(2) - denominator * inv); // inverse mod 2^256

        // Because the division is now exact we can divide by multiplying
        // with the modular inverse of denominator
        Some(prod0 * inv)
    }

    /// Calculates ceil(a×b÷denominator) with full precision
    /// Throws if result overflows a uint256 or denominator == 0
    pub fn mul_div_rounding_up(a: U256, b: U256, denominator: U256) -> Option<U256> {
        let result = Self::mul_div(a, b, denominator)?;
        if (a * b) % denominator > U256::zero() {
            return result.checked_add(U256::one());
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_div() {
        let a = U256::from(3);
        let b = U256::from(4);
        let denominator = U256::from(2);
        assert_eq!(FullMath::mul_div(a, b, denominator), Some(U256::from(6)));
    }

    #[test]
    fn test_mul_div_rounding_up() {
        let a = U256::from(7);
        let b = U256::from(8);
        let denominator = U256::from(10);
        assert_eq!(FullMath::mul_div_rounding_up(a, b, denominator), Some(U256::from(6)));
    }

    #[test]
    fn test_mul_div_zero_denominator() {
        let a = U256::from(3);
        let b = U256::from(4);
        let denominator = U256::from(0);
        assert_eq!(FullMath::mul_div(a, b, denominator), None);
    }
} 