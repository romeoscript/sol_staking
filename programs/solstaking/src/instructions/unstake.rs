use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::StakingError;

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    // Validate amount
    require!(amount > 0, StakingError::InvalidAmount);

    let current_slot = Clock::get()?.slot;
    
    // Store the values we need before mutable borrow
    let locktime = ctx.accounts.staking_contract.locktime;
    let last_claim_slot = ctx.accounts.staking_contract.last_claim_slot;
    
    // Check if the locktime has passed
    require!(
        current_slot.saturating_sub(last_claim_slot) >= locktime,
        StakingError::StakingPeriodNotReached
    );

    // Verify sufficient balance in staking account
    require!(
        ctx.accounts.staking_token_account.amount >= amount,
        StakingError::InsufficientContractBalance
    );

    // Create the seeds with proper binding
    let vault = b"vault";
    let key = ctx.accounts.staking_contract.to_account_info().key();
    let bump = ctx.bumps.staking_token_account;
    
    let seeds = [
        vault.as_ref(),
        key.as_ref(),
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // Transfer tokens back to user
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.staking_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.staking_token_account.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, amount)?;

    // Update total staked amount at the end
    let staking_contract = &mut ctx.accounts.staking_contract;
    staking_contract.total_staked = staking_contract.total_staked.checked_sub(amount)
        .ok_or(StakingError::MathOverflow)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
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
        seeds = [b"vault", staking_contract.key().as_ref()],
        bump,
        constraint = staking_token_account.mint == staking_contract.token_mint @ StakingError::InvalidToken
    )]
    pub staking_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}