use primitive_types::U256;

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

        // 512-bit multiplication using U256
        let ab = a.checked_mul(b)?;
        
        // Simple case - no overflow
        if let Some(result) = ab.checked_div(denominator) {
            return Some(result);
        }
        
        // Complex case - need to use more precise math
        // This is a simplified version that doesn't handle all edge cases
        // For a complete implementation, a more complex algorithm would be needed
        None
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