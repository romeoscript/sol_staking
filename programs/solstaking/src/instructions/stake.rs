use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::StakingError;

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    // Validate amount
    require!(amount > 0, StakingError::InvalidAmount);

    // Verify sufficient balance
    require!(
        ctx.accounts.user_token_account.amount >= amount,
        StakingError::InsufficientFunds
    );

    // Transfer tokens to staking vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.staking_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;

    // Update total staked amount
    let staking_contract = &mut ctx.accounts.staking_contract;
    staking_contract.total_staked = staking_contract.total_staked.checked_add(amount)
        .ok_or(StakingError::MathOverflow)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub staking_contract: Account<'info, StakingContract>,

    #[account(
        mut,
        constraint = user_token_account.mint == staking_contract.token_mint @ StakingError::InvalidToken,
        constraint = user_token_account.owner == user.key() @ StakingError::InvalidTokenAccount
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = staking_token_account.mint == staking_contract.token_mint @ StakingError::InvalidToken
    )]
    pub staking_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}