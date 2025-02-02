use anchor_lang::prelude::*;

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
    pub const BASE_LEN: usize = 8 + // discriminator
        32 + // owner (Pubkey)
        4 +  // pool_name length prefix
        8 +  // total_staked
        8 +  // apr
        8 +  // locktime
        32 + // token_mint
        8;   // last_claim_slot

    pub fn space(pool_name_len: usize) -> usize {
        Self::BASE_LEN + pool_name_len // Add dynamic pool name length
    }
}

impl StakingContract {
    pub fn validate_pool_parameters(apr: u64, locktime: u64) -> Result<()> {
        require!(apr > 0 && apr <= 100, ErrorCode::InvalidAPR);
        require!(locktime > 0, ErrorCode::InvalidLocktime);
        Ok(())
    }

    pub fn validate_pool_name(pool_name: &str) -> Result<()> {
        require!(!pool_name.is_empty(), ErrorCode::InvalidPoolName);
        require!(pool_name.len() <= 32, ErrorCode::PoolNameTooLong); // Set a reasonable max length
        Ok(())
    }
}