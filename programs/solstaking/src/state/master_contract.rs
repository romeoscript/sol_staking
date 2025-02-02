use anchor_lang::prelude::*;

#[account]
pub struct MasterContract {
    pub owner: Pubkey,
    pub pool_count: u64,
}

impl MasterContract {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner (Pubkey)
        8;   // pool_count (u64)
}