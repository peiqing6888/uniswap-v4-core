use std::collections::HashMap;
use primitive_types::U256;
use ethers::types::Address;
use thiserror::Error;

/// ERC6909 令牌错误类型
#[derive(Debug, Error)]
pub enum ERC6909Error {
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Insufficient allowance")]
    InsufficientAllowance,
    
    #[error("Invalid spender")]
    InvalidSpender,
    
    #[error("Invalid sender")]
    InvalidSender,
    
    #[error("Invalid recipient")]
    InvalidRecipient,
}

/// ERC6909 令牌事件
#[derive(Debug, Clone)]
pub enum ERC6909Event {
    /// 从 `from` 向 `to` 转移 `amount` 个id为 `id` 的令牌
    Transfer {
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    },
    
    /// `owner` 授权 `spender` 使用id为 `id` 的令牌，数量为 `amount`
    Approval {
        owner: Address,
        spender: Address,
        id: U256,
        amount: U256,
    },
    
    /// `owner` 为 `operator` 设置了所有令牌的授权状态为 `approved`
    OperatorSet {
        owner: Address,
        operator: Address,
        approved: bool,
    },
}

/// ERC6909 令牌类型 - 实现多令牌标准
#[derive(Debug)]
pub struct ERC6909 {
    /// 余额映射 (owner, id) => balance
    balances: HashMap<(Address, U256), U256>,
    
    /// 授权映射 (owner, spender, id) => allowance
    allowances: HashMap<(Address, Address, U256), U256>,
    
    /// 操作员映射 (owner, operator) => approved
    operators: HashMap<(Address, Address), bool>,
    
    /// 事件历史 - 在实际实现中将被替换为区块链事件
    events: Vec<ERC6909Event>,
}

impl ERC6909 {
    /// 创建一个新的ERC6909令牌实例
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            allowances: HashMap::new(),
            operators: HashMap::new(),
            events: Vec::new(),
        }
    }
    
    /// 查询代币余额
    pub fn balance_of(&self, owner: Address, id: U256) -> U256 {
        *self.balances.get(&(owner, id)).unwrap_or(&U256::zero())
    }
    
    /// 查询授权额度
    pub fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        *self.allowances.get(&(owner, spender, id)).unwrap_or(&U256::zero())
    }
    
    /// 查询操作员状态
    pub fn is_operator(&self, owner: Address, operator: Address) -> bool {
        *self.operators.get(&(owner, operator)).unwrap_or(&false)
    }
    
    /// 授权操作员
    pub fn set_operator(&mut self, caller: Address, operator: Address, approved: bool) -> Result<(), ERC6909Error> {
        if caller == Address::zero() {
            return Err(ERC6909Error::InvalidSender);
        }
        
        if operator == Address::zero() {
            return Err(ERC6909Error::InvalidSpender);
        }
        
        self.operators.insert((caller, operator), approved);
        
        // 触发事件
        self.events.push(ERC6909Event::OperatorSet {
            owner: caller,
            operator,
            approved,
        });
        
        Ok(())
    }
    
    /// 授权代币
    pub fn approve(&mut self, caller: Address, spender: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        if caller == Address::zero() {
            return Err(ERC6909Error::InvalidSender);
        }
        
        if spender == Address::zero() {
            return Err(ERC6909Error::InvalidSpender);
        }
        
        self.allowances.insert((caller, spender, id), amount);
        
        // 触发事件
        self.events.push(ERC6909Event::Approval {
            owner: caller,
            spender,
            id,
            amount,
        });
        
        Ok(())
    }
    
    /// 转移代币
    pub fn transfer(&mut self, caller: Address, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        if caller == Address::zero() {
            return Err(ERC6909Error::InvalidSender);
        }
        
        if to == Address::zero() {
            return Err(ERC6909Error::InvalidRecipient);
        }
        
        self._transfer(caller, caller, to, id, amount)
    }
    
    /// 授权转移代币
    pub fn transfer_from(&mut self, caller: Address, from: Address, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        if caller == Address::zero() {
            return Err(ERC6909Error::InvalidSender);
        }
        
        if from == Address::zero() {
            return Err(ERC6909Error::InvalidSender);
        }
        
        if to == Address::zero() {
            return Err(ERC6909Error::InvalidRecipient);
        }
        
        // 检查授权
        if from != caller && !self.is_operator(from, caller) {
            let allowed = self.allowance(from, caller, id);
            if allowed < amount {
                return Err(ERC6909Error::InsufficientAllowance);
            }
            
            // 更新授权额度
            self.allowances.insert((from, caller, id), allowed - amount);
        }
        
        self._transfer(caller, from, to, id, amount)
    }
    
    /// 铸造代币
    pub fn mint(&mut self, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        if to == Address::zero() {
            return Err(ERC6909Error::InvalidRecipient);
        }
        
        // 增加接收方余额
        let balance = self.balance_of(to, id);
        self.balances.insert((to, id), balance + amount);
        
        // 触发事件
        self.events.push(ERC6909Event::Transfer {
            from: Address::zero(),
            to,
            id,
            amount,
        });
        
        Ok(())
    }
    
    /// 销毁代币
    pub fn burn(&mut self, caller: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        if caller == Address::zero() {
            return Err(ERC6909Error::InvalidSender);
        }
        
        // 检查余额
        let balance = self.balance_of(caller, id);
        if balance < amount {
            return Err(ERC6909Error::InsufficientBalance);
        }
        
        // 减少余额
        self.balances.insert((caller, id), balance - amount);
        
        // 触发事件
        self.events.push(ERC6909Event::Transfer {
            from: caller,
            to: Address::zero(),
            id,
            amount,
        });
        
        Ok(())
    }
    
    /// 内部转移实现
    fn _transfer(&mut self, operator: Address, from: Address, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        // 检查余额
        let from_balance = self.balance_of(from, id);
        if from_balance < amount {
            return Err(ERC6909Error::InsufficientBalance);
        }
        
        // 更新余额
        self.balances.insert((from, id), from_balance - amount);
        
        let to_balance = self.balance_of(to, id);
        self.balances.insert((to, id), to_balance + amount);
        
        // 触发事件
        self.events.push(ERC6909Event::Transfer {
            from,
            to,
            id,
            amount,
        });
        
        Ok(())
    }
}

/// 流动性令牌 - 基于ERC6909实现的Uniswap V4流动性令牌
#[derive(Debug)]
pub struct LiquidityToken {
    /// 底层的ERC6909实现
    erc6909: ERC6909,
    
    /// 令牌名称
    name: String,
    
    /// 令牌符号
    symbol: String,
}

impl LiquidityToken {
    /// 创建一个新的流动性令牌
    pub fn new(name: String, symbol: String) -> Self {
        Self {
            erc6909: ERC6909::new(),
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
    
    /// 铸造流动性令牌 - 用于添加流动性
    pub fn mint_liquidity_token(&mut self, to: Address, pool_id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.erc6909.mint(to, pool_id, amount)
    }
    
    /// 销毁流动性令牌 - 用于移除流动性
    pub fn burn_liquidity_token(&mut self, from: Address, pool_id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.erc6909.burn(from, pool_id, amount)
    }
    
    /// 获取流动性令牌余额
    pub fn balance_of(&self, owner: Address, pool_id: U256) -> U256 {
        self.erc6909.balance_of(owner, pool_id)
    }
    
    /// 委托所有ERC6909函数
    pub fn transfer(&mut self, caller: Address, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.erc6909.transfer(caller, to, id, amount)
    }
    
    pub fn transfer_from(&mut self, caller: Address, from: Address, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.erc6909.transfer_from(caller, from, to, id, amount)
    }
    
    pub fn approve(&mut self, caller: Address, spender: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        self.erc6909.approve(caller, spender, id, amount)
    }
    
    pub fn set_operator(&mut self, caller: Address, operator: Address, approved: bool) -> Result<(), ERC6909Error> {
        self.erc6909.set_operator(caller, operator, approved)
    }
    
    pub fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        self.erc6909.allowance(owner, spender, id)
    }
    
    pub fn is_operator(&self, owner: Address, operator: Address) -> bool {
        self.erc6909.is_operator(owner, operator)
    }
} 