use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use crate::state::*;
use crate::errors::StakingError;

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::InvalidAmount);

    let current_slot = Clock::get()?.slot;
    let locktime = ctx.accounts.staking_contract.locktime;
    let last_claim_slot = ctx.accounts.staking_contract.last_claim_slot;
    
    // Check if early withdrawal
    let is_early_withdrawal = current_slot.saturating_sub(last_claim_slot) < locktime;
    
    // Calculate penalty (50% if early withdrawal)
    let (withdraw_amount, burn_amount) = if is_early_withdrawal {
        let penalty = amount.checked_div(2).ok_or(StakingError::MathOverflow)?;
        (penalty, penalty)
    } else {
        (amount, 0)
    };

    require!(
        ctx.accounts.staking_token_account.amount >= amount,
        StakingError::InsufficientContractBalance
    );

    let vault = b"vault";
    let key = ctx.accounts.staking_contract.to_account_info().key();
    let bump = ctx.bumps.staking_token_account;
    
    let seeds = [vault.as_ref(), key.as_ref(), &[bump]];
    let signer_seeds = &[&seeds[..]];

    // Transfer tokens to user
    if withdraw_amount > 0 {
        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.staking_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.staking_token_account.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, withdraw_amount)?;
    }

    // Burn penalty tokens if early withdrawal
    if burn_amount > 0 {
        let burn_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.staking_token_account.to_account_info(),
                authority: ctx.accounts.staking_token_account.to_account_info(),
            },
            signer_seeds,
        );
        token::burn(burn_ctx, burn_amount)?;
    }

    let staking_contract = &mut ctx.accounts.staking_contract;
    staking_contract.total_staked = staking_contract.total_staked
        .checked_sub(amount)
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

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}