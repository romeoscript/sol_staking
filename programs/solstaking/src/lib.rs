use anchor_lang::prelude::*;

declare_id!("FTNGc2wqb3oL1F3VZcqN6Ym99h5LuBPj8quG9YVnbz16");

#[program]
pub mod solstaking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
