use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::state::*;

pub fn create_staking_contract(
    ctx: Context<CreateStakingContract>,
    pool_name: String,
    token_mint: Pubkey,
    apr: u64,
    locktime: u64,
) -> Result<()> {
    // Validate parameters
    StakingContract::validate_pool_parameters(apr, locktime)?;
    StakingContract::validate_pool_name(&pool_name)?;

    let staking_contract = &mut ctx.accounts.staking_contract;
    staking_contract.owner = ctx.accounts.owner.key();
    staking_contract.pool_name = pool_name;
    staking_contract.total_staked = 0;
    staking_contract.apr = apr;
    staking_contract.locktime = locktime;
    staking_contract.token_mint = token_mint;
    staking_contract.last_claim_slot = Clock::get()?.slot;

    // Increment pool count in master contract
    let master_contract = &mut ctx.accounts.master_contract;
    master_contract.pool_count = master_contract.pool_count.checked_add(1).unwrap();

    Ok(())
}

#[derive(Accounts)]
#[instruction(pool_name: String)]
pub struct CreateStakingContract<'info> {
    #[account(mut)]
    pub master_contract: Account<'info, MasterContract>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = StakingContract::space(pool_name.len()),
        seeds = [b"staking", pool_name.as_bytes()],
        bump
    )]
    pub staking_contract: Account<'info, StakingContract>,

    #[account(
        init,
        payer = owner,
        token::mint = token_mint,
        token::authority = staking_token_account,
        seeds = [b"vault", staking_contract.key().as_ref()],
        bump
    )]
    pub staking_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, anchor_spl::token::Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}