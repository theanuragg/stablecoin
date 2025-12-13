use anchor_lang::prelude::*;
use anchor_spl::associated_token::spl_associated_token_account::solana_program::native_token::LAMPORTS_PER_SOL;
use pyth_solana_receiver_sdk::price_update::{FeedId, PriceUpdateV2, get_feed_id_from_hex};

use crate::{constraints::{FEED_ID, MAXIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT}, state::{Collateral, Config, CustomError}};

pub fn check_health_factor(
    Collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>
) -> Result<()> {
    let health_factor = calculate_health_factor(Collateral, config, price_feed)?;

    require!(
        health_factor >= config.min_health_factor,
        CustomError::BelowMinimumHealthFactor
    );

    Ok(())
}

pub fn calculate_health_factor(
    Collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>
) -> u64 {
    let  Collateral_value_in_usd = get_usd_value(&Collateral.lamport_balance, price_feed)?;

    let Collateral_adjusted_for_liquidation_thresshold =  (Collateral_value_in_usd + config.liquidition_threshold)/100;

    if Collateral.amount_minted == 0 {
        msg!("No tokens minted yet, returning max health factor");
        return Ok(u64::MAX);
        }


    let health_factor = (Collateral_adjusted_for_liquidation_thresshold / Collateral.amount_minted);
    Ok(health_factor)
}

pub fn get_usd_value(
   amount_in_lamports: &u64,
    price_feed: &Account<PriceUpdateV2>
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID);

    let price = price_feed.get_price_no_older_than(clock&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    require!(price.price> 0 , CustomError::InvalidPrice);

    let  price_in_usd = price.price as  u126 * PRICE_FEED_DECIMAL_ADJUSTMENT;
    let amount_in_usd = (*amount_in_lamports as  u126 * price_in_usd) /(LAMPORTS_PER_SOL as u126);

    Ok(amount_in_usd as u64)
}