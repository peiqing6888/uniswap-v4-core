use primitive_types::U256;
use crate::core::math::{MathError, Result, BitMath};

/// Functions for handling tick-related math
pub struct TickMath;

impl TickMath {
    /// The minimum tick that can be used on any pool
    pub const MIN_TICK: i32 = -887272;
    /// The maximum tick that can be used on any pool
    pub const MAX_TICK: i32 = 887272;

    /// The minimum tick spacing allowed by the protocol
    pub const MIN_TICK_SPACING: i32 = 1;
    /// The maximum tick spacing allowed by the protocol
    pub const MAX_TICK_SPACING: i32 = 32767;

    /// The minimum sqrt price as a Q64.96
    pub const MIN_SQRT_PRICE: U256 = U256([4295128739, 0, 0, 0]);
    /// The maximum sqrt price as a Q64.96
    pub const MAX_SQRT_PRICE: U256 = U256([
        6743328256752651558,
        17280870778742802505,
        4294805859,
        0,
    ]);

    /// Returns the maximum tick that can be used with the given tick spacing
    pub fn max_usable_tick(tick_spacing: i32) -> i32 {
        (Self::MAX_TICK / tick_spacing) * tick_spacing
    }

    /// Returns the minimum tick that can be used with the given tick spacing
    pub fn min_usable_tick(tick_spacing: i32) -> i32 {
        (Self::MIN_TICK / tick_spacing) * tick_spacing
    }

    /// Returns the sqrt price for the given tick as a Q64.96
    pub fn get_sqrt_price_at_tick(tick: i32) -> Result<U256> {
        if tick < Self::MIN_TICK || tick > Self::MAX_TICK {
            return Err(MathError::InvalidTick);
        }

        let abs_tick = if tick < 0 { -tick } else { tick } as u32;

        // 使用字符串初始化大整数，确保使用十六进制前缀
        let mut ratio = if abs_tick & 0x1 != 0 {
            U256::from_str_radix("fffcb933bd6fad37aa2d162d1a594001", 16).unwrap_or(U256::one() << 96)
        } else {
            U256::one() << 96
        };

        if abs_tick & 0x2 != 0 {
            ratio = (ratio * U256::from_str_radix("fff97272373d413259a46990580e213a", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x4 != 0 {
            ratio = (ratio * U256::from_str_radix("fff2e50f5f656932ef12357cf3c7fdcc", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x8 != 0 {
            ratio = (ratio * U256::from_str_radix("ffe5caca7e10e4e61c3624eaa0941cd0", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x10 != 0 {
            ratio = (ratio * U256::from_str_radix("ffcb9843d60f6159c9db58835c926644", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x20 != 0 {
            ratio = (ratio * U256::from_str_radix("ff973b41fa98c081472e6896dfb254c0", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x40 != 0 {
            ratio = (ratio * U256::from_str_radix("ff2ea16466c96a3843ec78b326b52861", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x80 != 0 {
            ratio = (ratio * U256::from_str_radix("fe5dee046a99a2a811c461f1969c3053", 16).unwrap_or(U256::one())) >> 128;
        }
        // 添加其他位操作
        if abs_tick & 0x100 != 0 {
            ratio = (ratio * U256::from_str_radix("fcbe86c7900a88aedcffc83b479aa3a4", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x200 != 0 {
            ratio = (ratio * U256::from_str_radix("f987a7253ac413176f2b074cf7815e54", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x400 != 0 {
            ratio = (ratio * U256::from_str_radix("f3392b0822b70005940c7a398e4b70f3", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x800 != 0 {
            ratio = (ratio * U256::from_str_radix("e7159475a2c29b7443b29c7fa6e889d9", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x1000 != 0 {
            ratio = (ratio * U256::from_str_radix("d097f3bdfd2022b8845ad8f792aa5825", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x2000 != 0 {
            ratio = (ratio * U256::from_str_radix("a9f746462d870fdf8a65dc1f90e061e5", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x4000 != 0 {
            ratio = (ratio * U256::from_str_radix("70d869a156d2a1b890bb3df62baf32f7", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x8000 != 0 {
            ratio = (ratio * U256::from_str_radix("31be135f97d08fd981231505542fcfa6", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x10000 != 0 {
            ratio = (ratio * U256::from_str_radix("9aa508b5b7a84e1c677de54f3e99bc9", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x20000 != 0 {
            ratio = (ratio * U256::from_str_radix("5d6af8dedb81196699c329225ee604", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x40000 != 0 {
            ratio = (ratio * U256::from_str_radix("2216e584f5fa1ea926041bedfe98", 16).unwrap_or(U256::one())) >> 128;
        }
        if abs_tick & 0x80000 != 0 {
            ratio = (ratio * U256::from_str_radix("48a170391f7dc42444e8fa2", 16).unwrap_or(U256::one())) >> 128;
        }

        if tick > 0 {
            ratio = U256::MAX / ratio;
        }

        // This divides by 1<<32 rounding up to go from a Q128.128 to a Q128.96.
        // We then downcast to Q64.96 if the result fits.
        let divisor = U256::from(1u64 << 32); // 使用 u64 避免溢出
        let sqrt_price_x96 = if ratio % divisor == U256::zero() {
            ratio >> 32
        } else {
            (ratio >> 32) + U256::one()
        };

        if sqrt_price_x96 > Self::MAX_SQRT_PRICE {
            return Err(MathError::InvalidPrice);
        }

        Ok(sqrt_price_x96)
    }

    /// Returns the tick corresponding to the given sqrt price as a Q64.96
    pub fn get_tick_at_sqrt_price(sqrt_price_x96: U256) -> Result<i32> {
        if sqrt_price_x96 < Self::MIN_SQRT_PRICE || sqrt_price_x96 > Self::MAX_SQRT_PRICE {
            return Err(MathError::InvalidPrice);
        }

        let price = sqrt_price_x96;
        let msb = BitMath::most_significant_bit(price);
        
        // 避免使用大位移，改用乘法
        let mut log_2 = (msb as i64) * (1i64 << 32) * 2;  // 相当于 msb << 64，但避免溢出

        let mut r = if msb >= 128 {
            price >> (msb - 127)
        } else {
            price << (127 - msb)
        };

        // Use repeated squaring to compute the log base sqrt(1.0001)
        for i in 0..14 {
            r = (r * r) >> 127;
            let f = r >> 128;
            // 将 U256 转换为 u32，再转为 i64，避免直接从 U256 转为 i32
            let f_u32 = f.low_u32();
            log_2 = log_2 | ((f_u32 as i64) << (63 - i));
            r = r >> f;
        }

        // 使用 U256 来处理大整数计算
        let log_sqrt10001_u256 = U256::from(log_2 as u64) * U256::from_str_radix("255738958999603826347141", 10).unwrap_or(U256::one());
        let offset_down = U256::from_str_radix("3402992956809132418596140100660247210", 10).unwrap_or(U256::one());
        let offset_up = U256::from_str_radix("291339464771989622907027621153398088495", 10).unwrap_or(U256::one());
        
        let tick_low = ((log_sqrt10001_u256.overflowing_sub(offset_down).0) >> 128).low_u32() as i32;
        let tick_high = ((log_sqrt10001_u256.overflowing_add(offset_up).0) >> 128).low_u32() as i32;

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
            (1, (U256::from(1) << 96).checked_mul(U256::from(1000050)).unwrap_or(U256::one()) / U256::from(1000000)),
            (-1, (U256::from(1) << 96).checked_mul(U256::from(999950)).unwrap_or(U256::one()) / U256::from(1000000)),
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
            ((U256::from(1) << 96).checked_mul(U256::from(1000050)).unwrap_or(U256::one()) / U256::from(1000000), 1),
            ((U256::from(1) << 96).checked_mul(U256::from(999950)).unwrap_or(U256::one()) / U256::from(1000000), -1),
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
            Err(MathError::InvalidTick)
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