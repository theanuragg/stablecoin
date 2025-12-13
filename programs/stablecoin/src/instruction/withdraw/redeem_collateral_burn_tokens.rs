use anchor_lang::prelude::*;
use anchor_spl::token2022::{Token2022, Mint, TokenAccount};
use anchor_spl::token_interface::{InterfaceAccount, Interface};
use crate::instruction::check_health_factor;
use crate::{Collateral, Config, SEED_CONFIG_ACCOUNT, SEED_COLLATERAL_ACCOUNT};

#[derive(Accounts)]
pub struct RedeemCollateralBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    pub price_update: AccountInfo<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump= config_account.bump,
        has_one = mint_account,
    )]
    pub  config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump = collateral_account.bump,
        has_one = depositor,
        has_one = sol_account,
    )]
    pub collateral_account: Account<'info, Collateral>,
    #[account(mut)]
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
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;
    collateral_account.amount_minted -= amount_to_burn;



    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update
    );

    Ok(())
}       