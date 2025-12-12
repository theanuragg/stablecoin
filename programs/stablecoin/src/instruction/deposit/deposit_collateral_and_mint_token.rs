use anchor_lang::prelude::*;
use anchor_spl::{associated_token::spl_associated_token_account::solana_program::example_mocks::solana_account::Account, token_interface::{Mint, Token2022, TokenAccount, }};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{Config, Collateral, SEED_CONFIG_ACCOUNT, SEED_COLLATERAL_ACCOUNT, SEED_SOL_ACCOUNT};



#[derive(Accounts)]
pub struct DepositCollateralAndMintToken<'info> {

    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds =[SEED_CONFIG_ACCOUNT],
        bump= config_account.bump,
        has_one = mint_account,

    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = depositor,
        space= 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint =mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
    pub price_update: Account<'info, PriceUpdate>,
    

}


pub fn process_deposit_collateral_and_mint_token(
    ctx: Context<DepositCollateralAndMintToken>,
    amount_to_mint: u64,
    amount_collateral: u64,   
) -> Result<()> {
    

    let Collateral_account: &mut Account<'info, Collateral> = &mut ctx.accounts.collateral_account;
    Collateral_account.lamports_balance += amount_collateral;
    Collateral_account.amount_minted += amount_to_mint;

    if !Collateral_account.is_initialized {
        Collateral_account.depositor = ctx.accounts.depositor.key();
        Collateral_account.sol_amount = ctx.accounts.sol_account.key();
        Collateral_account.token_amount = ctx.accounts.token_account.key();
        Collateral_account.bump = ctx.bumps.collateral_account;
        Collateral_account.is_initialized = true;
    }
    Ok(())
}