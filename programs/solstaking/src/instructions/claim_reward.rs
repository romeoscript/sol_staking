use anchor_lang::prelude::*;
use crate::state::*;

pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    let staking_contract = &mut ctx.accounts.staking_contract;
    
    // Verify stake exists
    require!(staking_contract.total_staked > 0, ErrorCode::NoStake);

    // Calculate slots elapsed since last claim
    let current_slot = Clock::get()?.slot;
    let slots_elapsed = current_slot.saturating_sub(staking_contract.last_claim_slot);
    
    // Check if minimum locktime has been reached
    require!(
        slots_elapsed >= staking_contract.locktime,
        ErrorCode::StakingPeriodNotReached
    );

    // Calculate SOL reward
    let reward = calculate_reward(
        staking_contract.total_staked,
        staking_contract.apr,
        slots_elapsed,
    )?;

    require!(reward > 0, ErrorCode::RewardTooSmall);
    
    // Verify reward vault has enough SOL
    require!(
        ctx.accounts.reward_vault.lamports() >= reward,
        ErrorCode::InsufficientContractBalance
    );

    // Transfer SOL from vault to user
    **ctx.accounts.reward_vault.try_borrow_mut_lamports()? = ctx
        .accounts
        .reward_vault
        .lamports()
        .checked_sub(reward)
        .ok_or(ErrorCode::MathOverflow)?;

    **ctx.accounts.user.try_borrow_mut_lamports()? = ctx
        .accounts
        .user
        .lamports()
        .checked_add(reward)
        .ok_or(ErrorCode::MathOverflow)?;

    // Update last claim slot
    staking_contract.last_claim_slot = current_slot;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"staking", staking_contract.pool_name.as_bytes()],
        bump
    )]
    pub staking_contract: Account<'info, StakingContract>,

    /// CHECK: This is safe because we only use it to transfer SOL
    #[account(
        mut,
        seeds = [b"reward_vault", staking_contract.key().as_ref()],
        bump
    )]
    pub reward_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

fn calculate_reward(
    total_staked: u64,
    apr: u64,
    slots_elapsed: u64,
) -> Result<u64> {
    let slots_per_year = 63_072_000; // Approximate number of slots in a year
    
    let reward = (total_staked as u128)
        .checked_mul(apr as u128)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_mul(slots_elapsed as u128)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(slots_per_year as u128)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(100) // APR is in percentage
        .ok_or(ErrorCode::MathOverflow)?;

    Ok(reward as u64)
}