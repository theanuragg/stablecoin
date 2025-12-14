//redeem_collateral_and_burn_token.rs
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

// Fixed: Import from utils, not instruction
use super::{burn_tokens, withdraw_sol};
use crate::instruction::check_health_factor;
use crate::{Collateral, Config, SEED_CONFIG_ACCOUNT, SEED_COLLATERAL_ACCOUNT, SEED_SOL_ACCOUNT};

#[derive(Accounts)]
pub struct RedeemCollateralBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    
    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = depositor,
        // Removed has_one = sol_account since we verify it manually via seeds
    )]
    pub collateral_account: Account<'info, Collateral>,
    
    /// CHECK: This is a PDA that holds SOL collateral
    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,
    
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

pub fn process_redeem_collateral_and_burn_tokens(
    ctx: Context<RedeemCollateralBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    
    // Update collateral account balances
    collateral_account.lamport_amount -= amount_collateral;
    collateral_account.amount_minted -= amount_to_burn;

    // Check health factor after updating balances
    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update
    )?;

    // Burn tokens first
    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        amount_to_burn,
        &ctx.accounts.depositor,
    )?;

    // Then withdraw SOL
    // Use the bump from RedeemCollateralBurnTokens context, not from collateral_account
    let sol_account_bump = ctx.bumps.sol_account;
    
    withdraw_sol(
        sol_account_bump,
        &ctx.accounts.depositor.key(),
        &ctx.accounts.system_program,
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor.to_account_info(),
        amount_collateral,
    )?;

    Ok(())
}

