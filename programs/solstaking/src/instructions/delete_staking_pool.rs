use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::StakingError;

pub fn delete_staking_pool(
    ctx: Context<DeleteStakingPool>,
    force: bool
) -> Result<()> {
    require!(
        ctx.accounts.staking_contract.owner == ctx.accounts.owner.key(),
        StakingError::Unauthorized
    );

    if !force {
        require!(
            ctx.accounts.staking_contract.total_staked == 0,
            StakingError::ActiveStakes
        );
    } else {
        // Force deletion - return staked tokens to owner
        if ctx.accounts.staking_contract.total_staked > 0 {
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.staking_token_account.to_account_info(),
                        to: ctx.accounts.owner_token_account.to_account_info(),
                        authority: ctx.accounts.staking_token_account.to_account_info(),
                    },
                    &[&[
                        b"vault",
                        ctx.accounts.staking_contract.key().as_ref(),
                        &[ctx.bumps.staking_token_account],
                    ]],
                ),
                ctx.accounts.staking_contract.total_staked
            )?;
        }
    }

    // Close token account
    token::close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::CloseAccount {
                account: ctx.accounts.staking_token_account.to_account_info(),
                destination: ctx.accounts.owner.to_account_info(),
                authority: ctx.accounts.staking_token_account.to_account_info(),
            },
            &[&[
                b"vault",
                ctx.accounts.staking_contract.key().as_ref(),
                &[ctx.bumps.staking_token_account],
            ]],
        ),
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(force: bool)]
pub struct DeleteStakingPool<'info> {
    #[account(mut, close = owner)]
    pub staking_contract: Account<'info, StakingContract>,

    #[account(
        mut,
        seeds = [b"vault", staking_contract.key().as_ref()],
        bump
    )]
    pub staking_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}