use anchor_lang::prelude::*;

declare_id!("3t1GKEdWt7Dnt8XzkJkHrCGtDoqyZytC4n56yoKnaK8w");

#[program]
pub mod solana_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
