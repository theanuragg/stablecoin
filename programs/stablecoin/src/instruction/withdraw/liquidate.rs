use anchor_lang::prelude::*;
use anchor_spl::{ token_interface::{Mint, Token2022, TokenAccount}};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::{Collateral, Config, CustomError, constraints::SEED_CONFIG_ACCOUNT, instruction::{calculate_health_factor, get_lamport_from_usd}};

#[derive(accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds= [SEED_CONFIG_ACCOUNT],
        bump= config_account.bump,
        has_one= mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(mut)]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

pub fn process_liquidate(
    ctx: Context<liquidate>,
    amount_collateral: u64,
    amount_to_burn: u64,
) -> Result<()> {

    let health_factor = calculate_health_factor(
        &ctx.accounts.collateral_account, 
        &ctx.accounts.config_account, 
        &ctx.accounts.price_update
    )?;

    require!(
        health_factor < ctx.accounts.config_account.min_health_factor,
        CustomError::HealthyAccountCannotBeLiquidated
    );

    let lamports = get_lamport_from_usd(&amount_to_burn, &ctx.accounts.price_update)?;

    let liquidation_bonus = lamports * ctx.accounts.config_account.liquidition_bonus / 100;

    let amount_to_liquidate = lamports + liquidation_bonus;

    let collateral_account = &mut ctx.accounts.collateral_account;

    // Withdraw SOL collateral to liquidator
    withdraw_sol(
        ctx.accounts.collateral_account.bump_sol_account,
        &ctx.accounts.collateral_account.depositor,
        &ctx.accounts.system_program,
        &ctx.accounts.sol_account,
        &ctx.accounts.liquidator.to_account_info(),
        amount_to_liquidate,
    )?;

    // Burn stablecoin tokens from liquidator
    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        amount_to_burn,
        &ctx.accounts.liquidator,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_amount -= ctx.accounts.sol_account.lamports();
    collateral_account.amount_minted -= amount_to_burn;

    check_health_factor(
        &ctx.accounts.price_update,
        &ctx.accounts.config_account,
        collateral_account,
    )?;

    Ok(())
}