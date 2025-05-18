use primitive_types::U256;
use crate::core::math::{MathError, Result};

/// BitMath library for bit operations
pub struct BitMath;

impl BitMath {
    /// Returns the index of the most significant bit of the number
    /// `x` must be greater than 0
    pub fn most_significant_bit(mut x: U256) -> u8 {
        assert!(x > U256::zero(), "BitMath: Zero value");
        
        // Binary search for the most significant bit
        let mut msb = 0;
        
        if x >= U256::from(1) << 128 {
            x = x >> 128;
            msb += 128;
        }
        
        if x >= U256::from(1) << 64 {
            x = x >> 64;
            msb += 64;
        }
        
        if x >= U256::from(1) << 32 {
            x = x >> 32;
            msb += 32;
        }
        
        if x >= U256::from(1) << 16 {
            x = x >> 16;
            msb += 16;
        }
        
        if x >= U256::from(1) << 8 {
            x = x >> 8;
            msb += 8;
        }
        
        if x >= U256::from(1) << 4 {
            x = x >> 4;
            msb += 4;
        }
        
        if x >= U256::from(1) << 2 {
            x = x >> 2;
            msb += 2;
        }
        
        if x >= U256::from(1) << 1 {
            msb += 1;
        }
        
        msb
    }

    /// Returns the index of the least significant bit of the number
    /// `x` must be greater than 0
    pub fn least_significant_bit(mut x: U256) -> u8 {
        assert!(x > U256::zero(), "BitMath: Zero value");
        
        // Binary search for the least significant bit
        let mut lsb = 255;
        
        if (x & ((U256::from(1) << 128) - U256::from(1))) > U256::zero() {
            lsb -= 128;
        } else {
            x = x >> 128;
        }
        
        if (x & ((U256::from(1) << 64) - U256::from(1))) > U256::zero() {
            lsb -= 64;
        } else {
            x = x >> 64;
        }
        
        if (x & ((U256::from(1) << 32) - U256::from(1))) > U256::zero() {
            lsb -= 32;
        } else {
            x = x >> 32;
        }
        
        if (x & ((U256::from(1) << 16) - U256::from(1))) > U256::zero() {
            lsb -= 16;
        } else {
            x = x >> 16;
        }
        
        if (x & ((U256::from(1) << 8) - U256::from(1))) > U256::zero() {
            lsb -= 8;
        } else {
            x = x >> 8;
        }
        
        if (x & ((U256::from(1) << 4) - U256::from(1))) > U256::zero() {
            lsb -= 4;
        } else {
            x = x >> 4;
        }
        
        if (x & ((U256::from(1) << 2) - U256::from(1))) > U256::zero() {
            lsb -= 2;
        } else {
            x = x >> 2;
        }
        
        if (x & U256::from(1)) > U256::zero() {
            lsb -= 1;
        }
        
        lsb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_significant_bit() {
        assert_eq!(BitMath::most_significant_bit(U256::from(1)), 0);
        assert_eq!(BitMath::most_significant_bit(U256::from(2)), 1);
        assert_eq!(BitMath::most_significant_bit(U256::from(4)), 2);
        assert_eq!(BitMath::most_significant_bit(U256::from(8)), 3);
        assert_eq!(BitMath::most_significant_bit(U256::from(255)), 7);
        assert_eq!(BitMath::most_significant_bit(U256::from(256)), 8);
    }

    #[test]
    fn test_least_significant_bit() {
        assert_eq!(BitMath::least_significant_bit(U256::from(1)), 0);
        assert_eq!(BitMath::least_significant_bit(U256::from(2)), 1);
        assert_eq!(BitMath::least_significant_bit(U256::from(4)), 2);
        assert_eq!(BitMath::least_significant_bit(U256::from(8)), 3);
        assert_eq!(BitMath::least_significant_bit(U256::from(9)), 0);
        assert_eq!(BitMath::least_significant_bit(U256::from(10)), 1);
    }

    #[test]
    #[should_panic(expected = "BitMath: Zero value")]
    fn test_most_significant_bit_zero() {
        BitMath::most_significant_bit(U256::zero());
    }

    #[test]
    #[should_panic(expected = "BitMath: Zero value")]
    fn test_least_significant_bit_zero() {
        BitMath::least_significant_bit(U256::zero());
    }
} 