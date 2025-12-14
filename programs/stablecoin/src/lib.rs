use anchor_lang::prelude::*;

// Declare modules first
pub mod state;
pub mod constraints;
pub mod instruction;
pub mod error;

// Then import what you need
use state::*;
use constraints::*;
use instruction::*;
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
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_token(
            ctx,
            amount_collateral,
            amount_to_mint,
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

    pub fn liquidate(
        ctx: Context<Liquidate>,
        amount_collateral: u64,
        amount_to_burn: u64,
    ) -> Result<()> {
        process_liquidate(
            ctx,
            amount_collateral,
            amount_to_burn,
        )
    }
}