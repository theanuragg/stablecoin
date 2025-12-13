use anchor_lang::{accounts::signer, prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{token_2022::{mint_to, MintTo, Token2022},token_interface::{Mint, TokenAccount}};

pub fn mint_tokens<'info>(
    mint_account: &Interface<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: Program<'info, Token2022>,
    amount: u64,
    bump: u8,
) -> Result<()>{

    let signer_seeds: &[&[u8]] = &[
        SEED_MINT_ACCOUNT,
        &[bump],
    ];

    mint_to(
        CpiContext::new_with_signer(
            program::token_program.to_account_info(),
            MintTo{
                mint: mint_account.to_account_info(), 
                to: token_account.to_account_info(),
                authority: mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    );
}


pub fn  desposit_sol<'info>(
    from; &signer<'info>,
    to: &SystemAccount<'info>,
    system_program: Program<'info, System>,
    
) -> Result<()> {
   transfer(CpiContext::new(program: system_program.to_account_info(), 
   accounts: Transfer{
    from: from.to_account_info(),
    to: to.to_account_info(),
   },
), 
}