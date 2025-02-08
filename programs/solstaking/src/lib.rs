use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("7nRA853Bg6xAqdZ3iqxiMhXV3ZC2tSU3gGHjcUFGHCNp");

#[program]
pub mod solstaking {
    use super::*;

    pub fn initialize_master(ctx: Context<InitializeMaster>, owner: Pubkey) -> Result<()> {
        instructions::initialize_master(ctx, owner)
    }

    pub fn create_staking_contract(
        ctx: Context<CreateStakingContract>,
        pool_name: String,
        token_mint: Pubkey,
        apr: u64,
        locktime: u64,
    ) -> Result<()> {
        instructions::create_staking_contract(ctx, pool_name, token_mint, apr, locktime)
    }

    pub fn modify_pool_parameters(
        ctx: Context<ModifyPoolParameters>,
        new_apr: u64,
        new_locktime: u64,
    ) -> Result<()> {
        instructions::modify_pool_parameters(ctx, new_apr, new_locktime)
    }

    pub fn delete_staking_pool(ctx: Context<DeleteStakingPool>, force: bool) -> Result<()> {
        instructions::delete_staking_pool(ctx, force)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::unstake(ctx, amount)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward(ctx)
    }

    pub fn view_unclaimed_rewards(ctx: Context<ViewRewards>) -> Result<u64> {
        instructions::view_unclaimed_rewards(ctx)
    }
}