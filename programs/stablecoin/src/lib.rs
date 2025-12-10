use anchor_lang::prelude::*;

declare_id!("7MKHVRryd6nmqHJsCrhsHUxfU3gnUxAd5Zn2jnpFPMaL");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
