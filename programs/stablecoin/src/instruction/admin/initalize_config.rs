use anchor_lang::prelude::*;
use crate::{Config, MINT_DECIMAL, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT, LIQUIDITION_BONUS, LiQUIDITION_THRESHOLD, MIN_HEALTH_FACTOR};
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
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
// Function to process the initialize config instruction
pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
   *ctx.accounts.config_account= Config { 
    authority: ctx.accounts.authority.key(),
    mint_account: ctx.accounts.mint_account.key(),
    liquidition_threshold: LiQUIDITION_THRESHOLD, 
    liquidition_bonus: LIQUIDITION_BONUS, 
    min_health_factor: MIN_HEALTH_FACTOR, 
    bump: ctx.bumps.config_account,
    bump_mint_account: ctx.bumps.mint_account,

    };
   
    Ok(())
}
