use anchor_lang::prelude::*;
use crate::{Config, MINT_DECIMAL, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT};

use anchor_spl::token_interface::{Mint, Token2022};
// Instruction to initialize the config account
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + Config::INIT_SPACE,)]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMAL,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: AccountInfo<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
// Function to process the initialize config instruction
pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    Ok(())
}
