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

    pub fn deposit_collateral_and_mint_token(
        ctx: Context<DepositCollateralAndMintToken>,
        amount_to_deposit: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_token(
            ctx,
            amount_to_deposit,
            amount_to_mint,
        )
    }
}
