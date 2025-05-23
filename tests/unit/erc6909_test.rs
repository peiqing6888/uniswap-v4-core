#[cfg(test)]
mod erc6909_tests {
    use ethers::types::Address;
    use primitive_types::U256;
    use uniswap_v4_core::tokens::{
        ERC6909, LiquidityToken, ERC6909Error,
        LiquidityTokenClaims, ERC6909Claims, ClaimsError
    };

    #[test]
    fn test_erc6909_basic_operations() {
        let mut token = ERC6909::new();
        let owner = Address::random();
        let recipient = Address::random();
        let token_id = U256::from(1);
        let amount = U256::from(1000);

        // 测试铸造
        token.mint(recipient, token_id, amount).unwrap();
        assert_eq!(token.balance_of(recipient, token_id), amount);

        // 测试转账
        token.transfer(recipient, owner, token_id, U256::from(400)).unwrap();
        assert_eq!(token.balance_of(recipient, token_id), U256::from(600));
        assert_eq!(token.balance_of(owner, token_id), U256::from(400));

        // 测试授权
        token.approve(owner, recipient, token_id, U256::from(200)).unwrap();
        assert_eq!(token.allowance(owner, recipient, token_id), U256::from(200));

        // 测试授权转账
        token.transfer_from(recipient, owner, recipient, token_id, U256::from(200)).unwrap();
        assert_eq!(token.balance_of(owner, token_id), U256::from(200));
        assert_eq!(token.balance_of(recipient, token_id), U256::from(800));
        assert_eq!(token.allowance(owner, recipient, token_id), U256::from(0));

        // 测试设置操作员
        token.set_operator(owner, recipient, true).unwrap();
        assert!(token.is_operator(owner, recipient));

        // 测试使用操作员无需授权即可转账
        token.transfer_from(recipient, owner, recipient, token_id, U256::from(100)).unwrap();
        assert_eq!(token.balance_of(owner, token_id), U256::from(100));
        assert_eq!(token.balance_of(recipient, token_id), U256::from(900));

        // 测试销毁
        token.burn(recipient, token_id, U256::from(900)).unwrap();
        assert_eq!(token.balance_of(recipient, token_id), U256::from(0));
    }

    #[test]
    fn test_erc6909_error_conditions() {
        let mut token = ERC6909::new();
        let owner = Address::random();
        let recipient = Address::random();
        let token_id = U256::from(1);

        // 测试余额不足错误
        let result = token.transfer(owner, recipient, token_id, U256::from(100));
        assert!(matches!(result, Err(ERC6909Error::InsufficientBalance)));

        // 测试授权不足错误
        token.mint(owner, token_id, U256::from(1000)).unwrap();
        let result = token.transfer_from(
            recipient, owner, recipient, token_id, U256::from(100)
        );
        assert!(matches!(result, Err(ERC6909Error::InsufficientAllowance)));

        // 测试无效接收方错误
        let result = token.transfer(owner, Address::zero(), token_id, U256::from(100));
        assert!(matches!(result, Err(ERC6909Error::InvalidRecipient)));
    }

    #[test]
    fn test_liquidity_token() {
        let mut liquidity_token = LiquidityToken::new(
            "Uniswap V4 LP".to_string(),
            "UNI-V4-LP".to_string()
        );
        let owner = Address::random();
        let pool_id = U256::from(1);
        let amount = U256::from(1000);

        // 测试基本属性
        assert_eq!(liquidity_token.name(), "Uniswap V4 LP");
        assert_eq!(liquidity_token.symbol(), "UNI-V4-LP");

        // 测试铸造流动性令牌
        liquidity_token.mint_liquidity_token(owner, pool_id, amount).unwrap();
        assert_eq!(liquidity_token.balance_of(owner, pool_id), amount);

        // 测试转移流动性令牌
        let recipient = Address::random();
        liquidity_token.transfer(owner, recipient, pool_id, U256::from(400)).unwrap();
        assert_eq!(liquidity_token.balance_of(owner, pool_id), U256::from(600));
        assert_eq!(liquidity_token.balance_of(recipient, pool_id), U256::from(400));

        // 测试销毁流动性令牌
        liquidity_token.burn_liquidity_token(owner, pool_id, U256::from(600)).unwrap();
        assert_eq!(liquidity_token.balance_of(owner, pool_id), U256::from(0));
    }

    #[test]
    fn test_erc6909_claims() {
        let mut claims = ERC6909Claims::new();
        let owner = Address::random();
        let recipient = Address::random();
        let token_id = U256::from(1);
        let amount = U256::from(1000);

        // 铸造令牌
        claims.erc6909_mut().mint(owner, token_id, amount).unwrap();
        assert_eq!(claims.erc6909().balance_of(owner, token_id), amount);

        // 创建声明
        let claim_id = claims.create_claim(owner, recipient, token_id, U256::from(500)).unwrap();
        
        // 验证声明存在
        let claim = claims.get_claim(claim_id).unwrap();
        assert_eq!(claim.owner, owner);
        assert_eq!(claim.recipient, recipient);
        assert_eq!(claim.token_id, token_id);
        assert_eq!(claim.amount, U256::from(500));

        // 取消声明
        claims.cancel_claim(owner, claim_id).unwrap();
        assert!(claims.get_claim(claim_id).is_none());

        // 再次创建声明
        let claim_id = claims.create_claim(owner, recipient, token_id, U256::from(300)).unwrap();
        
        // 授权 recipient 从 owner 处转移令牌
        claims.erc6909_mut().approve(owner, recipient, token_id, U256::from(300)).unwrap();

        // 执行声明
        claims.execute_claim(recipient, claim_id).unwrap();
        assert!(claims.get_claim(claim_id).is_none());
        assert_eq!(claims.erc6909().balance_of(owner, token_id), U256::from(700));
        assert_eq!(claims.erc6909().balance_of(recipient, token_id), U256::from(300));
    }

    #[test]
    fn test_liquidity_token_claims() {
        let mut liquidity_claims = LiquidityTokenClaims::new(
            "Uniswap V4 LP Claims".to_string(),
            "UNI-V4-LP-C".to_string()
        );
        let owner = Address::random();
        let recipient = Address::random();
        let pool_id = U256::from(1);
        let amount = U256::from(1000);

        // 测试基本属性
        assert_eq!(liquidity_claims.name(), "Uniswap V4 LP Claims");
        assert_eq!(liquidity_claims.symbol(), "UNI-V4-LP-C");

        // 铸造流动性令牌
        liquidity_claims.mint_liquidity_token(owner, pool_id, amount).unwrap();
        assert_eq!(liquidity_claims.balance_of(owner, pool_id), amount);

        // 创建流动性声明
        let claim_id = liquidity_claims.create_liquidity_claim(
            owner, recipient, pool_id, U256::from(500)
        ).unwrap();

        // 验证声明是否存在
        // 首先，owner 需要授权 recipient 来执行声明中的转移
        liquidity_claims.approve(owner, recipient, pool_id, U256::from(500)).unwrap();
        
        let result = liquidity_claims.execute_liquidity_claim(recipient, claim_id);
        assert!(result.is_ok());

        // 验证余额变化
        assert_eq!(liquidity_claims.balance_of(owner, pool_id), U256::from(500));
        assert_eq!(liquidity_claims.balance_of(recipient, pool_id), U256::from(500));
    }
} 