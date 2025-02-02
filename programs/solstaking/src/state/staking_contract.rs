use anchor_lang::prelude::*;
use crate::errors::StakingError;

#[account]
pub struct StakingContract {
    pub owner: Pubkey,
    pub pool_name: String,
    pub total_staked: u64,
    pub apr: u64,
    pub locktime: u64,
    pub token_mint: Pubkey,
    pub last_claim_slot: u64,
}

impl StakingContract {
    pub const BASE_LEN: usize = 8 + 
        32 + 
        4 +  
        8 +  
        8 + 
        8 +  
        32 + 
        8;   

    pub fn space(pool_name_len: usize) -> usize {
        Self::BASE_LEN + pool_name_len // Add dynamic pool name length
    }
}

impl StakingContract {
    pub fn validate_pool_parameters(apr: u64, locktime: u64) -> Result<()> {
        require!(apr > 0 && apr <= 100, StakingError::InvalidAPR);
        require!(locktime > 0, StakingError::InvalidLocktime);
        Ok(())
    }

    pub fn validate_pool_name(pool_name: &str) -> Result<()> {
        require!(!pool_name.is_empty(), StakingError::InvalidPoolName);
        require!(pool_name.len() <= 32, StakingError::PoolNameTooLong); // Set a reasonable max length
        Ok(())
    }
}