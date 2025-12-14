use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount}
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    Collateral, 
    Config, 
    SEED_COLLATERAL_ACCOUNT, 
    SEED_CONFIG_ACCOUNT, 
    SEED_SOL_ACCOUNT,
    instruction::{check_health_factor, deposit_sol, mint_tokens}
};

#[derive(Accounts)]
pub struct DepositCollateralAndMintToken<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub collateral_account: Account<'info, Collateral>,
    
    /// CHECK: This is a PDA used to hold SOL collateral
    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,
    
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn process_deposit_collateral_and_mint_token(
    ctx: Context<DepositCollateralAndMintToken>,
    amount_collateral: u64,
    amount_to_mint: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    
    // Update collateral account state FIRST (before health check)
    collateral_account.lamport_amount += amount_collateral;
    collateral_account.amount_minted += amount_to_mint;

    if !collateral_account.is_initialized {
        collateral_account.depositor = ctx.accounts.depositor.key();
        collateral_account.sol_amount = ctx.accounts.sol_account.key();
        collateral_account.token_amount = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_account;
        collateral_account.is_initialized = true;
    }

    // Check health factor AFTER updating the state
    check_health_factor(
        collateral_account,  // Don't use &ctx.accounts here since we already have the reference
        &ctx.accounts.config_account, 
        &ctx.accounts.price_update
    )?;

    // Deposit SOL
    deposit_sol(
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        &ctx.accounts.system_program,
        amount_collateral,  // Use the parameter directly
    )?;

    // Mint tokens
    mint_tokens(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        amount_to_mint,  // Use the parameter directly
        ctx.accounts.config_account.bump_mint_account,
    )?;
    
    Ok(())
}