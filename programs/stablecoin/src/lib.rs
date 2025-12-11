use anchor_lang::prelude::*;
use state::*;
mod state;
use constraints::*;
mod constraints;
use instruction::*;
mod instruction;

declare_id!("7MKHVRryd6nmqHJsCrhsHUxfU3gnUxAd5Zn2jnpFPMaL");
#[program]
pub mod stablecoin {
    use super::*;
   pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        min_health_factor: u64,
    ) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
}
