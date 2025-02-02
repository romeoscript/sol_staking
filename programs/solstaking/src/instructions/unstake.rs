use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    // Validate amount
    require!(amount > 0, ErrorCode::InvalidAmount);

    // Check if the locktime has passed
    let current_slot = Clock::get()?.slot;
    let staking_contract = &mut ctx.accounts.staking_contract;
    
    require!(
        current_slot.saturating_sub(staking_contract.last_claim_slot) >= staking_contract.locktime,
        ErrorCode::StakingPeriodNotReached
    );

    // Verify sufficient balance in staking account
    require!(
        ctx.accounts.staking_token_account.amount >= amount,
        ErrorCode::InsufficientContractBalance
    );

    // Transfer tokens back to user
    let staking_seeds = &[
        b"vault".as_ref(),
        ctx.accounts.staking_contract.to_account_info().key.as_ref(),
        &[ctx.bumps.staking_token_account],
    ];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.staking_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.staking_token_account.to_account_info(),
        },
        &[staking_seeds],
    );
    token::transfer(transfer_ctx, amount)?;

    // Update total staked amount
    staking_contract.total_staked = staking_contract.total_staked.checked_sub(amount)
        .ok_or(ErrorCode::MathOverflow)?;

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
        constraint = user_token_account.mint == staking_contract.token_mint @ ErrorCode::InvalidToken,
        constraint = user_token_account.owner == user.key() @ ErrorCode::InvalidTokenAccount
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", staking_contract.key().as_ref()],
        bump,
        constraint = staking_token_account.mint == staking_contract.token_mint @ ErrorCode::InvalidToken
    )]
    pub staking_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}