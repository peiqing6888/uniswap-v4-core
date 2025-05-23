use std::collections::HashMap;
use primitive_types::U256;
use ethers::types::Address;
use super::erc6909::{ERC6909, ERC6909Error};

/// Claims错误类型
#[derive(Debug, thiserror::Error)]
pub enum ClaimsError {
    #[error("Invalid claim owner")]
    InvalidClaimOwner,
    
    #[error("Invalid claim recipient")]
    InvalidClaimRecipient,
    
    #[error("Invalid claim ID")]
    InvalidClaimId,
    
    #[error("Claim already exists")]
    ClaimAlreadyExists,
    
    #[error("Claim does not exist")]
    ClaimDoesNotExist,
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Unauthorized caller")]
    Unauthorized,
}

/// 令牌声明事件
#[derive(Debug, Clone)]
pub enum ClaimsEvent {
    /// 创建了新的声明
    ClaimCreated {
        owner: Address,
        recipient: Address,
        id: U256,
        amount: U256,
        claim_id: U256,
    },
    
    /// 取消了声明
    ClaimCancelled {
        claim_id: U256,
    },
    
    /// 执行了声明
    ClaimExecuted {
        claim_id: U256,
    },
}

/// 令牌声明结构
#[derive(Debug, Clone)]
pub struct Claim {
    /// 声明的所有者
    pub owner: Address,
    
    /// 声明的接收者
    pub recipient: Address,
    
    /// 令牌ID
    pub token_id: U256,
    
    /// 令牌数量
    pub amount: U256,
}

/// ERC6909 Claims扩展 - 允许创建和执行令牌声明
#[derive(Debug)]
pub struct ERC6909Claims {
    /// 底层的ERC6909实现
    erc6909: ERC6909,
    
    /// 声明映射 claim_id => Claim
    claims: HashMap<U256, Claim>,
    
    /// 下一个声明ID
    next_claim_id: U256,
    
    /// 事件历史
    events: Vec<ClaimsEvent>,
}

impl ERC6909Claims {
    /// 创建一个新的ERC6909Claims实例
    pub fn new() -> Self {
        Self {
            erc6909: ERC6909::new(),
            claims: HashMap::new(),
            next_claim_id: U256::one(),
            events: Vec::new(),
        }
    }
    
    /// 获取底层的ERC6909实现
    pub fn erc6909(&self) -> &ERC6909 {
        &self.erc6909
    }
    
    /// 获取底层的ERC6909实现的可变引用
    pub fn erc6909_mut(&mut self) -> &mut ERC6909 {
        &mut self.erc6909
    }
    
    /// 创建一个新的令牌声明
    pub fn create_claim(
        &mut self,
        caller: Address,
        recipient: Address,
        token_id: U256,
        amount: U256
    ) -> Result<U256, ClaimsError> {
        // 验证参数
        if caller == Address::zero() {
            return Err(ClaimsError::InvalidClaimOwner);
        }
        
        if recipient == Address::zero() {
            return Err(ClaimsError::InvalidClaimRecipient);
        }
        
        // 检查余额
        let balance = self.erc6909.balance_of(caller, token_id);
        if balance < amount {
            return Err(ClaimsError::InsufficientBalance);
        }
        
        // 创建新声明
        let claim_id = self.next_claim_id;
        self.next_claim_id += U256::one();
        
        let claim = Claim {
            owner: caller,
            recipient,
            token_id,
            amount,
        };
        
        self.claims.insert(claim_id, claim);
        
        // 触发事件
        self.events.push(ClaimsEvent::ClaimCreated {
            owner: caller,
            recipient,
            id: token_id,
            amount,
            claim_id,
        });
        
        Ok(claim_id)
    }
    
    /// 取消一个令牌声明
    pub fn cancel_claim(&mut self, caller: Address, claim_id: U256) -> Result<(), ClaimsError> {
        // 检查声明是否存在
        let claim = self.claims.get(&claim_id).ok_or(ClaimsError::ClaimDoesNotExist)?;
        
        // 验证调用者是否为声明所有者
        if claim.owner != caller {
            return Err(ClaimsError::Unauthorized);
        }
        
        // 移除声明
        self.claims.remove(&claim_id);
        
        // 触发事件
        self.events.push(ClaimsEvent::ClaimCancelled {
            claim_id,
        });
        
        Ok(())
    }
    
    /// 执行一个令牌声明
    pub fn execute_claim(&mut self, caller: Address, claim_id: U256) -> Result<(), ClaimsError> {
        // 检查声明是否存在
        let claim = self.claims.get(&claim_id).ok_or(ClaimsError::ClaimDoesNotExist)?;
        
        // 验证调用者是否为声明接收者
        if claim.recipient != caller {
            return Err(ClaimsError::Unauthorized);
        }
        
        // 执行转移
        match self.erc6909.transfer_from(
            caller,
            claim.owner,
            claim.recipient,
            claim.token_id,
            claim.amount
        ) {
            Ok(_) => {},
            Err(ERC6909Error::InsufficientBalance) => return Err(ClaimsError::InsufficientBalance),
            Err(ERC6909Error::InsufficientAllowance) => return Err(ClaimsError::Unauthorized), // Or a new ClaimsError::InsufficientAllowance
            Err(_) => return Err(ClaimsError::Unauthorized), // Or a more generic error
        };
        
        // 移除声明
        self.claims.remove(&claim_id);
        
        // 触发事件
        self.events.push(ClaimsEvent::ClaimExecuted {
            claim_id,
        });
        
        Ok(())
    }
    
    /// 获取声明详情
    pub fn get_claim(&self, claim_id: U256) -> Option<&Claim> {
        self.claims.get(&claim_id)
    }
}

/// 流动性令牌声明 - 基于ERC6909Claims实现的Uniswap V4流动性令牌声明
#[derive(Debug)]
pub struct LiquidityTokenClaims {
    /// 底层的ERC6909Claims实现
    claims: ERC6909Claims,
    
    /// 令牌名称
    name: String,
    
    /// 令牌符号
    symbol: String,
}

impl LiquidityTokenClaims {
    /// 创建一个新的流动性令牌声明
    pub fn new(name: String, symbol: String) -> Self {
        Self {
            claims: ERC6909Claims::new(),
            name,
            symbol,
        }
    }
    
    /// 获取令牌名称
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// 获取令牌符号
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
    
    /// 铸造流动性令牌
    pub fn mint_liquidity_token(&mut self, to: Address, pool_id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.claims.erc6909_mut().mint(to, pool_id, amount)
    }
    
    /// 销毁流动性令牌
    pub fn burn_liquidity_token(&mut self, from: Address, pool_id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.claims.erc6909_mut().burn(from, pool_id, amount)
    }
    
    /// 获取流动性令牌余额
    pub fn balance_of(&self, owner: Address, pool_id: U256) -> U256 {
        self.claims.erc6909().balance_of(owner, pool_id)
    }
    
    /// 创建流动性令牌声明
    pub fn create_liquidity_claim(
        &mut self,
        caller: Address,
        recipient: Address,
        pool_id: U256,
        amount: U256
    ) -> Result<U256, ClaimsError> {
        self.claims.create_claim(caller, recipient, pool_id, amount)
    }
    
    /// 取消流动性令牌声明
    pub fn cancel_liquidity_claim(&mut self, caller: Address, claim_id: U256) -> Result<(), ClaimsError> {
        self.claims.cancel_claim(caller, claim_id)
    }
    
    /// 执行流动性令牌声明
    pub fn execute_liquidity_claim(&mut self, caller: Address, claim_id: U256) -> Result<(), ClaimsError> {
        self.claims.execute_claim(caller, claim_id)
    }
    
    /// 委托所有ERC6909函数
    pub fn transfer(&mut self, caller: Address, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.claims.erc6909_mut().transfer(caller, to, id, amount)
    }
    
    pub fn transfer_from(&mut self, caller: Address, from: Address, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.claims.erc6909_mut().transfer_from(caller, from, to, id, amount)
    }
    
    pub fn approve(&mut self, caller: Address, spender: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.claims.erc6909_mut().approve(caller, spender, id, amount)
    }
    
    pub fn set_operator(&mut self, caller: Address, operator: Address, approved: bool) -> Result<(), ERC6909Error> {
        self.claims.erc6909_mut().set_operator(caller, operator, approved)
    }
    
    pub fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        self.claims.erc6909().allowance(owner, spender, id)
    }
    
    pub fn is_operator(&self, owner: Address, operator: Address) -> bool {
        self.claims.erc6909().is_operator(owner, operator)
    }
} 