use anchor_lang::prelude::*;
use crate::state::*;

pub fn initialize_master(ctx: Context<InitializeMaster>, owner: Pubkey) -> Result<()> {
    let master_contract = &mut ctx.accounts.master_contract;
    
    master_contract.owner = owner;
    master_contract.pool_count = 0;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMaster<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = MasterContract::LEN,
        seeds = [b"master"],
        bump
    )]
    pub master_contract: Account<'info, MasterContract>,

    pub system_program: Program<'info, System>,
}