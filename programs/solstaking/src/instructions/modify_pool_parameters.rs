use anchor_lang::prelude::*;
use crate::state::*;

pub fn modify_pool_parameters(
    ctx: Context<ModifyPoolParameters>,
    new_apr: u64,
    new_locktime: u64,
) -> Result<()> {
    // Validate new parameters
    StakingContract::validate_pool_parameters(new_apr, new_locktime)?;

    // Verify owner
    require!(
        ctx.accounts.staking_contract.owner == ctx.accounts.owner.key(),
        ErrorCode::Unauthorized
    );

    let staking_contract = &mut ctx.accounts.staking_contract;
    staking_contract.apr = new_apr;
    staking_contract.locktime = new_locktime;

    Ok(())
}

#[derive(Accounts)]
pub struct ModifyPoolParameters<'info> {
    #[account(mut)]
    pub staking_contract: Account<'info, StakingContract>,

    #[account(mut)]
    pub owner: Signer<'info>,
}