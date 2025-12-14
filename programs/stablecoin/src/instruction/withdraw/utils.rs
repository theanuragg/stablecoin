use anchor_lang::{accounts::{signer, system_account}, prelude::{system_instruction::transfer, *}};

use crate::constraints::SEED_SOL_ACCOUNT;

pub fn withdraw_sol(
   bump: u8,
   depositor_key: &Pubkey,
   system_program: &Program<'info>,
   from: &SystemAccount<'info>,
   to: &AccountInfo<'info>,
) -> Result<()> {
    let signer_seeds: &[&[u8]] = &[
        SEED_SOL_ACCOUNT,
        depositor_key.as_ref(),
        &[bump],
    ];

    transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            Transfer{
                from: from.to_account_info(),
                to: to.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
    Ok(())
}

pub fn burn_tokens<'info>(
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

    burn(
        CpiContext::new_with_signer(
            program::token_program.to_account_info(),
            Burn{
                mint: mint_account.to_account_info(), 
                from: token_account.to_account_info(),
                authority: mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    );
    Ok(())
}