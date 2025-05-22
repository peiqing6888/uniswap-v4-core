#[cfg(test)]
mod protocol_fee_tests {
    use ethers::types::Address;
    use primitive_types::U256;
    use uniswap_v4_core::fees::{
        ProtocolFee, ProtocolFeeManager, ProtocolFeesAccrued,
        types::MAX_PROTOCOL_FEE, ProtocolFeeIntegration
    };
    use uniswap_v4_core::core::flash_loan::currency::Currency;
    use uniswap_v4_core::core::hooks::hook_interface::PoolKey;
    use uniswap_v4_core::core::state::Pool;

    #[test]
    fn test_protocol_fee_creation() {
        // Test creating protocol fee from zero
        let fee = ProtocolFee::new(0, 0);
        assert_eq!(fee.get_zero_for_one_fee(), 0);
        assert_eq!(fee.get_one_for_zero_fee(), 0);
        assert!(fee.is_valid());

        // Test setting maximum protocol fee
        let max_fee = ProtocolFee::new(MAX_PROTOCOL_FEE, MAX_PROTOCOL_FEE);
        assert_eq!(max_fee.get_zero_for_one_fee(), MAX_PROTOCOL_FEE);
        assert_eq!(max_fee.get_one_for_zero_fee(), MAX_PROTOCOL_FEE);
        assert!(max_fee.is_valid());

        // Test different directions of protocol fee
        let asymmetric_fee = ProtocolFee::new(100, 200);
        assert_eq!(asymmetric_fee.get_zero_for_one_fee(), 100);
        assert_eq!(asymmetric_fee.get_one_for_zero_fee(), 200);
        assert!(asymmetric_fee.is_valid());
    }

    #[test]
    fn test_protocol_fee_calculation() {
        let fee = ProtocolFee::new(100, 200); // 0.01% for 0->1, 0.02% for 1->0
        let lp_fee = 3000; // 0.3%

        // Test zero-for-one direction protocol fee calculation
        let zero_for_one_swap_fee = fee.calculate_swap_fee(true, lp_fee);
        // Expected result: 3000 + 100 - (3000 * 100 / 1_000_000) = 3100 - 0.3 = 3099.7 ≈ 3099
        assert_eq!(zero_for_one_swap_fee, 3099);

        // Test one-for-zero direction protocol fee calculation
        let one_for_zero_swap_fee = fee.calculate_swap_fee(false, lp_fee);
        // Expected result: 3000 + 200 - (3000 * 200 / 1_000_000) = 3200 - 0.6 = 3199.4 ≈ 3199
        assert_eq!(one_for_zero_swap_fee, 3199);
    }

    #[test]
    fn test_protocol_fee_manager() {
        let owner = Address::random();
        let mut manager = ProtocolFeeManager::new(owner);

        // Test setting protocol fee controller
        let new_controller = Address::random();
        manager.set_protocol_fee_controller(new_controller, owner).unwrap();
        assert_eq!(manager.controller, new_controller);

        // Test updating protocol fee
        let currency = Currency::from_address(Address::random());
        manager.update_protocol_fees(currency, U256::from(1000));
        assert_eq!(manager.protocol_fees_accrued(currency), U256::from(1000));

        // Test collecting protocol fee
        let collected = manager.collect_protocol_fees(
            new_controller, 
            Address::random(), 
            currency, 
            U256::zero(),
            false
        ).unwrap();
        assert_eq!(collected, U256::from(1000));
        assert_eq!(manager.protocol_fees_accrued(currency), U256::zero());
    }

    #[test]
    fn test_protocol_fee_integration() {
        let owner = Address::random();
        let mut integration = ProtocolFeeIntegration::new(owner);
        let mut pool = Pool::new();
        
        // Initialize pool
        let sqrt_price = uniswap_v4_core::core::math::types::SqrtPrice::new(
            U256::from(2).pow(U256::from(96))
        );
        pool.initialize(sqrt_price, 3000).unwrap();
        
        // Test updating swap fees
        let protocol_fee = ProtocolFee::new(100, 200); // 0.01% for 0->1, 0.02% for 1->0
        let amount_specified = -1_000_000i128; // Negative value means exactInput
        let currency = Currency::from_address(Address::random());
        
        // Update zero-for-one direction swap fees
        let fee_amount = integration.update_fees_for_swap(
            &mut pool, amount_specified, true, protocol_fee, currency
        );
        
        // Check if protocol fee is correctly collected and recorded
        assert!(fee_amount > 0);
        assert!(integration.manager.protocol_fees_accrued(currency) > U256::zero());
    }
} 