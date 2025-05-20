use ethers::{
    contract::{abigen, Contract},
    providers::{Http, Provider},
    types::{Address, U256, TransactionRequest},
    core::types::TransactionReceipt,
    signers::{LocalWallet, Signer},
    middleware::{SignerMiddleware, Middleware},
};
use std::sync::Arc;

// Generate bindings for ERC20 tokens
abigen!(
    IERC20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool)
        function approve(address spender, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
    ]"#,
);

/// TokenInteractor provides functionality to interact with ERC20 tokens
pub struct TokenInteractor {
    provider: Arc<Provider<Http>>,
    signer: Option<LocalWallet>,
    signed_provider: Option<Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>>,
}

impl TokenInteractor {
    /// Create a new TokenInteractor with just a provider (read-only)
    pub fn new(provider: Arc<Provider<Http>>) -> Self {
        Self {
            provider,
            signer: None,
            signed_provider: None,
        }
    }
    
    /// Create a new TokenInteractor with a provider and signer (read-write)
    pub fn with_signer(provider: Arc<Provider<Http>>, signer: LocalWallet) -> Self {
        let signed_provider = Arc::new(SignerMiddleware::new(provider.clone(), signer.clone()));
        Self {
            provider,
            signer: Some(signer),
            signed_provider: Some(signed_provider),
        }
    }
    
    /// Get the balance of a token for an address
    pub async fn balance_of(&self, token_address: Address, owner: Address) -> Result<U256, String> {
        let token = IERC20::new(token_address, self.provider.clone());
        token.balance_of(owner).call().await.map_err(|e| e.to_string())
    }
    
    /// Transfer tokens (requires signer)
    pub async fn transfer(&self, token_address: Address, recipient: Address, amount: U256) -> Result<bool, String> {
        let signed_provider = self.signed_provider.as_ref().ok_or("No signer provided")?;
        let token = IERC20::new(token_address, signed_provider.clone());
        
        let tx = token.transfer(recipient, amount);
        tx.call().await.map_err(|e| e.to_string())
    }
    
    /// Transfer tokens from one address to another (requires signer and approval)
    pub async fn transfer_from(&self, token_address: Address, sender: Address, recipient: Address, amount: U256) -> Result<bool, String> {
        let signed_provider = self.signed_provider.as_ref().ok_or("No signer provided")?;
        let token = IERC20::new(token_address, signed_provider.clone());
        
        let tx = token.transfer_from(sender, recipient, amount);
        tx.call().await.map_err(|e| e.to_string())
    }
    
    /// Approve tokens for spending (requires signer)
    pub async fn approve(&self, token_address: Address, spender: Address, amount: U256) -> Result<bool, String> {
        let signed_provider = self.signed_provider.as_ref().ok_or("No signer provided")?;
        let token = IERC20::new(token_address, signed_provider.clone());
        
        let tx = token.approve(spender, amount);
        tx.call().await.map_err(|e| e.to_string())
    }
    
    /// Get the allowance of a token for a spender from an owner
    pub async fn allowance(&self, token_address: Address, owner: Address, spender: Address) -> Result<U256, String> {
        let token = IERC20::new(token_address, self.provider.clone());
        token.allowance(owner, spender).call().await.map_err(|e| e.to_string())
    }
    
    /// Send a transaction and wait for it to be mined
    pub async fn send_transaction(&self, token_address: Address, tx_data: TransactionRequest) -> Result<Option<TransactionReceipt>, String> {
        let signed_provider = self.signed_provider.as_ref().ok_or("No signer provided")?;
        
        signed_provider.send_transaction(tx_data, None)
            .await
            .map_err(|e| e.to_string())?
            .await
            .map_err(|e| e.to_string())
    }
} 