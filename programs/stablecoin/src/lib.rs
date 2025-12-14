use anchor_lang::prelude::*;
use state::*;
mod state;
use constraints::*;
mod constraints;
use instruction::*;
mod instruction;
pub mod error;
use error::CustomError;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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
    amount_collateral: u64,  // ← FIRST
    amount_to_mint: u64,     // ← SECOND
) -> Result<()> {
    process_deposit_collateral_and_mint_token(
        ctx,
        amount_collateral,  // ← Pass FIRST
        amount_to_mint,     // ← Pass SECOND
    )
}

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(
            ctx,
            amount_collateral,
            amount_to_burn,
        )
    }
}
