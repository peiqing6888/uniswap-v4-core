use primitive_types::U256;
use crate::core::math::{MathError, Result};

/// BitMath provides functionality for computing bit properties of an unsigned integer
pub struct BitMath;

impl BitMath {
    /// Returns the index of the most significant bit of the number,
    /// where the least significant bit is at index 0 and the most significant bit is at index 255
    /// 
    /// # Arguments
    /// * `x` - The value for which to compute the most significant bit, must be greater than 0
    /// 
    /// # Returns
    /// * `Result<u8>` - The index of the most significant bit
    pub fn most_significant_bit(x: U256) -> Result<u8> {
        if x.is_zero() {
            return Err(MathError::InvalidPrice);
        }

        // Rust implementation using leading_zeros
        Ok((255 - x.leading_zeros()) as u8)
    }

    /// Returns the index of the least significant bit of the number,
    /// where the least significant bit is at index 0 and the most significant bit is at index 255
    /// 
    /// # Arguments
    /// * `x` - The value for which to compute the least significant bit, must be greater than 0
    /// 
    /// # Returns
    /// * `Result<u8>` - The index of the least significant bit
    pub fn least_significant_bit(x: U256) -> Result<u8> {
        if x.is_zero() {
            return Err(MathError::InvalidPrice);
        }

        // Rust implementation using trailing_zeros
        Ok(x.trailing_zeros() as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_significant_bit() {
        // Test cases
        let test_cases = vec![
            (U256::from(1), 0),
            (U256::from(2), 1),
            (U256::from(3), 1),
            (U256::from(4), 2),
            (U256::from(7), 2),
            (U256::from(8), 3),
            (U256::from(255), 7),
            (U256::from(256), 8),
            (U256::from(u64::MAX), 63),
        ];

        for (input, expected) in test_cases {
            assert_eq!(BitMath::most_significant_bit(input).unwrap(), expected);
        }
    }

    #[test]
    fn test_least_significant_bit() {
        // Test cases
        let test_cases = vec![
            (U256::from(1), 0),
            (U256::from(2), 1),
            (U256::from(3), 0),
            (U256::from(4), 2),
            (U256::from(7), 0),
            (U256::from(8), 3),
            (U256::from(256), 8),
            (U256::from(u64::MAX), 0),
        ];

        for (input, expected) in test_cases {
            assert_eq!(BitMath::least_significant_bit(input).unwrap(), expected);
        }
    }

    #[test]
    fn test_zero_input() {
        assert!(matches!(BitMath::most_significant_bit(U256::zero()), Err(MathError::InvalidPrice)));
        assert!(matches!(BitMath::least_significant_bit(U256::zero()), Err(MathError::InvalidPrice)));
    }
} 