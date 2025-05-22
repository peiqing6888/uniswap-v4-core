use ethers::types::Address;
use crate::core::state::Result as StateResult;
use crate::core::hooks::hook_interface::PoolKey;
use crate::core::flash_loan::Currency;
use super::types::{ProtocolFee, ProtocolFeesAccrued};
use primitive_types::U256;

/// Error types for protocol fee control
#[derive(Debug, thiserror::Error)]
pub enum ProtocolFeeError {
    #[error("Protocol fee too large")]
    ProtocolFeeTooLarge(u32),
    
    #[error("Invalid caller")]
    InvalidCaller,
    
    #[error("Protocol fee currency synced")]
    ProtocolFeeCurrencySynced,
}

/// Protocol fee controller interface
pub trait ProtocolFeeController {
    /// Set protocol fee
    fn set_protocol_fee(&mut self, key: &PoolKey, new_protocol_fee: ProtocolFee) -> Result<(), ProtocolFeeError>;
    
    /// Collect protocol fees
    fn collect_protocol_fees(
        &mut self, 
        recipient: Address, 
        currency: Currency, 
        amount: U256
    ) -> Result<U256, ProtocolFeeError>;
}

/// Protocol fee events
#[derive(Debug, Clone)]
pub enum ProtocolFeeEvent {
    /// Protocol fee controller update event
    ProtocolFeeControllerUpdated { controller: Address },
    
    /// Protocol fee update event
    ProtocolFeeUpdated { pool_id: [u8; 32], protocol_fee: u32 },
}

/// Protocol fee manager
#[derive(Debug)]
pub struct ProtocolFeeManager {
    /// Current protocol fee controller address
    pub controller: Address,
    
    /// Accrued protocol fees
    pub fees_accrued: ProtocolFeesAccrued,
    
    /// Event history (will be integrated with blockchain event system in actual implementation)
    pub events: Vec<ProtocolFeeEvent>,
}

impl ProtocolFeeManager {
    /// Create a new protocol fee manager
    pub fn new(initial_owner: Address) -> Self {
        Self {
            controller: initial_owner,
            fees_accrued: ProtocolFeesAccrued::new(),
            events: Vec::new(),
        }
    }
    
    /// Set protocol fee controller
    pub fn set_protocol_fee_controller(&mut self, controller: Address, caller: Address) -> Result<(), ProtocolFeeError> {
        // Check if the caller is the current controller or owner
        if caller != self.controller {
            return Err(ProtocolFeeError::InvalidCaller);
        }
        
        self.controller = controller;
        self.events.push(ProtocolFeeEvent::ProtocolFeeControllerUpdated { controller });
        
        Ok(())
    }
    
    /// Validate if protocol fee is valid
    pub fn validate_protocol_fee(&self, fee: ProtocolFee) -> Result<(), ProtocolFeeError> {
        if !fee.is_valid() {
            return Err(ProtocolFeeError::ProtocolFeeTooLarge(fee.0));
        }
        Ok(())
    }
    
    /// Set protocol fee for a pool
    pub fn set_protocol_fee(
        &mut self, 
        caller: Address, 
        key: &PoolKey, 
        new_protocol_fee: ProtocolFee,
        pool_id: [u8; 32]
    ) -> Result<(), ProtocolFeeError> {
        // Check if caller is the fee controller
        if caller != self.controller {
            return Err(ProtocolFeeError::InvalidCaller);
        }
        
        // Validate if fee is valid
        self.validate_protocol_fee(new_protocol_fee)?;
        
        // Record event
        self.events.push(ProtocolFeeEvent::ProtocolFeeUpdated { 
            pool_id, 
            protocol_fee: new_protocol_fee.0 
        });
        
        Ok(())
    }
    
    /// Update protocol fees
    pub fn update_protocol_fees(&mut self, currency: Currency, amount: U256) {
        let currency_address = currency.address().unwrap_or(Address::zero());
        
        self.fees_accrued.update_fees(currency_address, amount);
    }
    
    /// Query protocol fees
    pub fn protocol_fees_accrued(&self, currency: Currency) -> U256 {
        let currency_address = currency.address().unwrap_or(Address::zero());
        
        self.fees_accrued.get_fees(currency_address)
    }
    
    /// Collect protocol fees
    pub fn collect_protocol_fees(
        &mut self, 
        caller: Address,
        recipient: Address, 
        currency: Currency, 
        amount: U256,
        is_synced_currency: bool
    ) -> Result<U256, ProtocolFeeError> {
        // Check if caller is the fee controller
        if caller != self.controller {
            return Err(ProtocolFeeError::InvalidCaller);
        }
        
        // Prevent collecting fees on synced currency
        if !currency.is_native() && is_synced_currency {
            return Err(ProtocolFeeError::ProtocolFeeCurrencySynced);
        }
        
        let currency_address = currency.address().unwrap_or(Address::zero());
        
        let collected = self.fees_accrued.collect_fees(currency_address, amount);
        
        Ok(collected)
    }
} 