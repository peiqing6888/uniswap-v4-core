use primitive_types::U256;
use crate::core::math::{MathError, Result, BitMath};

/// Math library for computing sqrt prices from ticks and vice versa
pub struct TickMath;

impl TickMath {
    /// The minimum tick that may be passed to get_sqrt_price_at_tick
    pub const MIN_TICK: i32 = -887272;
    /// The maximum tick that may be passed to get_sqrt_price_at_tick
    pub const MAX_TICK: i32 = 887272;

    /// The minimum tick spacing
    pub const MIN_TICK_SPACING: i32 = 1;
    /// The maximum tick spacing
    pub const MAX_TICK_SPACING: i32 = 32767;

    /// The minimum value that can be returned from get_sqrt_price_at_tick
    pub const MIN_SQRT_PRICE: U256 = U256([4295128739, 0, 0, 0]);
    /// The maximum value that can be returned from get_sqrt_price_at_tick
    pub const MAX_SQRT_PRICE: U256 = U256([
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x1461446703485210,
    ]);

    /// Given a tick spacing, compute the maximum usable tick
    pub fn max_usable_tick(tick_spacing: i32) -> i32 {
        (Self::MAX_TICK / tick_spacing) * tick_spacing
    }

    /// Given a tick spacing, compute the minimum usable tick
    pub fn min_usable_tick(tick_spacing: i32) -> i32 {
        (Self::MIN_TICK / tick_spacing) * tick_spacing
    }

    /// Calculates sqrt(1.0001^tick) * 2^96
    pub fn get_sqrt_price_at_tick(tick: i32) -> Result<U256> {
        let abs_tick = tick.abs() as u32;
        if abs_tick > Self::MAX_TICK as u32 {
            return Err(MathError::InvalidPrice);
        }

        let mut price = if abs_tick & 0x1 != 0 {
            U256::from_dec_str("0xfffcb933bd6fad37aa2d162d1a594001").unwrap()
        } else {
            U256::from(1) << 128
        };

        // Apply the multipliers for each bit position
        if abs_tick & 0x2 != 0 { price = (price * U256::from_dec_str("0xfff97272373d413259a46990580e213a").unwrap()) >> 128; }
        if abs_tick & 0x4 != 0 { price = (price * U256::from_dec_str("0xfff2e50f5f656932ef12357cf3c7fdcc").unwrap()) >> 128; }
        if abs_tick & 0x8 != 0 { price = (price * U256::from_dec_str("0xffe5caca7e10e4e61c3624eaa0941cd0").unwrap()) >> 128; }
        if abs_tick & 0x10 != 0 { price = (price * U256::from_dec_str("0xffcb9843d60f6159c9db58835c926644").unwrap()) >> 128; }
        if abs_tick & 0x20 != 0 { price = (price * U256::from_dec_str("0xff973b41fa98c081472e6896dfb254c0").unwrap()) >> 128; }
        if abs_tick & 0x40 != 0 { price = (price * U256::from_dec_str("0xff2ea16466c96a3843ec78b326b52861").unwrap()) >> 128; }
        if abs_tick & 0x80 != 0 { price = (price * U256::from_dec_str("0xfe5dee046a99a2a811c461f1969c3053").unwrap()) >> 128; }
        if abs_tick & 0x100 != 0 { price = (price * U256::from_dec_str("0xfcbe86c7900a88aedcffc83b479aa3a4").unwrap()) >> 128; }
        if abs_tick & 0x200 != 0 { price = (price * U256::from_dec_str("0xf987a7253ac413176f2b074cf7815e54").unwrap()) >> 128; }
        if abs_tick & 0x400 != 0 { price = (price * U256::from_dec_str("0xf3392b0822b70005940c7a398e4b70f3").unwrap()) >> 128; }
        if abs_tick & 0x800 != 0 { price = (price * U256::from_dec_str("0xe7159475a2c29b7443b29c7fa6e889d9").unwrap()) >> 128; }
        if abs_tick & 0x1000 != 0 { price = (price * U256::from_dec_str("0xd097f3bdfd2022b8845ad8f792aa5825").unwrap()) >> 128; }
        if abs_tick & 0x2000 != 0 { price = (price * U256::from_dec_str("0xa9f746462d870fdf8a65dc1f90e061e5").unwrap()) >> 128; }
        if abs_tick & 0x4000 != 0 { price = (price * U256::from_dec_str("0x70d869a156d2a1b890bb3df62baf32f7").unwrap()) >> 128; }
        if abs_tick & 0x8000 != 0 { price = (price * U256::from_dec_str("0x31be135f97d08fd981231505542fcfa6").unwrap()) >> 128; }
        if abs_tick & 0x10000 != 0 { price = (price * U256::from_dec_str("0x9aa508b5b7a84e1c677de54f3e99bc9").unwrap()) >> 128; }
        if abs_tick & 0x20000 != 0 { price = (price * U256::from_dec_str("0x5d6af8dedb81196699c329225ee604").unwrap()) >> 128; }
        if abs_tick & 0x40000 != 0 { price = (price * U256::from_dec_str("0x2216e584f5fa1ea926041bedfe98").unwrap()) >> 128; }
        if abs_tick & 0x80000 != 0 { price = (price * U256::from_dec_str("0x48a170391f7dc42444e8fa2").unwrap()) >> 128; }

        if tick > 0 {
            price = U256::max_value() / price;
        }

        // Convert from Q128.128 to Q64.96
        Ok((price + ((1u64 << 32) - 1)) >> 32)
    }

    /// Calculates the greatest tick value such that get_sqrt_price_at_tick(tick) <= sqrt_price_x96
    pub fn get_tick_at_sqrt_price(sqrt_price_x96: U256) -> Result<i32> {
        if sqrt_price_x96 < Self::MIN_SQRT_PRICE || sqrt_price_x96 >= Self::MAX_SQRT_PRICE {
            return Err(MathError::InvalidPrice);
        }

        let price = sqrt_price_x96 << 32;
        let msb = BitMath::most_significant_bit(price)?;
        
        let mut log_2 = ((msb as i32) - 128) << 64;
        let mut r = if msb >= 128 {
            price >> (msb - 127)
        } else {
            price << (127 - msb)
        };

        // Use repeated squaring to compute the log base sqrt(1.0001)
        for i in 0..14 {
            r = (r * r) >> 127;
            let f = r >> 128;
            log_2 = log_2 | ((f as i32) << (63 - i));
            r = r >> f;
        }

        let log_sqrt10001 = log_2 * 255738958999603826347141; // 128.128 number

        let tick_low = ((log_sqrt10001 - 3402992956809132418596140100660247210) >> 128) as i32;
        let tick_high = ((log_sqrt10001 + 291339464771989622907027621153398088495) >> 128) as i32;

        let tick = if tick_low == tick_high {
            tick_low
        } else if Self::get_sqrt_price_at_tick(tick_high)? <= sqrt_price_x96 {
            tick_high
        } else {
            tick_low
        };

        if tick < Self::MIN_TICK || tick > Self::MAX_TICK {
            return Err(MathError::InvalidPrice);
        }

        Ok(tick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sqrt_price_at_tick() {
        // Test some known values
        let test_cases = vec![
            (0, U256::from(1) << 96),
            (1, U256::from(1000050000000000)),
            (-1, U256::from(999950000000000)),
        ];

        for (tick, expected) in test_cases {
            let result = TickMath::get_sqrt_price_at_tick(tick).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_get_tick_at_sqrt_price() {
        // Test some known values
        let test_cases = vec![
            (U256::from(1) << 96, 0),
            (U256::from(1000050000000000), 1),
            (U256::from(999950000000000), -1),
        ];

        for (sqrt_price, expected) in test_cases {
            let result = TickMath::get_tick_at_sqrt_price(sqrt_price).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_invalid_tick() {
        assert!(matches!(
            TickMath::get_sqrt_price_at_tick(TickMath::MAX_TICK + 1),
            Err(MathError::InvalidPrice)
        ));
    }

    #[test]
    fn test_invalid_sqrt_price() {
        assert!(matches!(
            TickMath::get_tick_at_sqrt_price(TickMath::MIN_SQRT_PRICE - U256::one()),
            Err(MathError::InvalidPrice)
        ));
    }
} 