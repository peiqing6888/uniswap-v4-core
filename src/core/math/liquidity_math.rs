use crate::core::math::{MathError, Result};

/// Math library for liquidity calculations
pub struct LiquidityMath;

impl LiquidityMath {
    /// Add a signed liquidity delta to liquidity and return error if it overflows or underflows
    /// 
    /// # Arguments
    /// * `x` - The liquidity before change
    /// * `y` - The delta by which liquidity should be changed
    /// 
    /// # Returns
    /// * `Result<u128>` - The new liquidity amount
    pub fn add_delta(x: u128, y: i128) -> Result<u128> {
        let z = if y < 0 {
            // For negative y, we need to ensure x >= abs(y) to prevent underflow
            let abs_y = y.unsigned_abs();
            if x < abs_y {
                return Err(MathError::Overflow);
            }
            x - abs_y
        } else {
            // For positive y, we need to ensure x + y <= u128::MAX to prevent overflow
            x.checked_add(y as u128).ok_or(MathError::Overflow)?
        };

        Ok(z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_delta_positive() {
        // Test adding positive delta
        let x = 1000u128;
        let y = 500i128;
        assert_eq!(LiquidityMath::add_delta(x, y).unwrap(), 1500);
    }

    #[test]
    fn test_add_delta_negative() {
        // Test adding negative delta
        let x = 1000u128;
        let y = -500i128;
        assert_eq!(LiquidityMath::add_delta(x, y).unwrap(), 500);
    }

    #[test]
    fn test_add_delta_zero() {
        // Test adding zero delta
        let x = 1000u128;
        let y = 0i128;
        assert_eq!(LiquidityMath::add_delta(x, y).unwrap(), 1000);
    }

    #[test]
    fn test_add_delta_overflow() {
        // Test overflow case
        let x = u128::MAX;
        let y = 1i128;
        assert!(matches!(LiquidityMath::add_delta(x, y), Err(MathError::Overflow)));
    }

    #[test]
    fn test_add_delta_underflow() {
        // Test underflow case
        let x = 100u128;
        let y = -200i128;
        assert!(matches!(LiquidityMath::add_delta(x, y), Err(MathError::Overflow)));
    }

    #[test]
    fn test_add_delta_max_values() {
        // Test with maximum values
        let x = u128::MAX;
        let y = 0i128;
        assert_eq!(LiquidityMath::add_delta(x, y).unwrap(), u128::MAX);

        let x = u128::MAX;
        let y = -1i128;
        assert_eq!(LiquidityMath::add_delta(x, y).unwrap(), u128::MAX - 1);
    }
} 
 