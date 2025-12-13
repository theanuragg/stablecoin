use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{FeedId, PriceUpdateV2, get_feed_id_from_hex};

use crate::{constraints::{FEED_ID, MAXIMUM_AGE}, state::{Collateral, Config}};

pub fn calculate_health_factor(
    Collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>
) -> u64 {
    let  Collateral_value_in_usd = get_usd_value(&Collateral.lamport_balance, price_feed)?;

}

pub fn get_usd_value(
   amount_in_lamports: &u64,
    price_feed: &Account<PriceUpdateV2>
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID);

    let price = price_feed.get_price_no_older_than(clock&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    require!(price.price> 0 , CustomError::InvalidPrice);
}