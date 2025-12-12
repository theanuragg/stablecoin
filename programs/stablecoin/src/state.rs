use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
     pub depositor: Pubkey,
     pub sol_amount: Pubkey,
     pub token_amount: Pubkey,
     pub lamport_amount: u64,
     pub amount_minted: u64,
     pub bump: u8,
     pub bump_sol_account: u8,
     pub is_initialized: bool,

}

#[account]
#[derive(InitSpace, Debug)]

pub struct Config {
    pub authority: Pubkey,
    pub mint_account: Pubkey,
    pub liquidition_threshold: u64,
    pub liquidition_bonus: u64,
    pub min_health_factor: u64,
    pub bump: u8,
    pub bump_mint_account: u8,

}