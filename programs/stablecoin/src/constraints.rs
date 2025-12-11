use anchor_lang::prelude::*;

pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config_account";
pub const  SEED_MINT_ACCOUNT: &[u8] = b"mint_account";

pub const MINT_DECIMAL: u8 = 9;
pub const LiQUIDITION_THRESHOLD: u64 = 50; // 75%
pub const LIQUIDITION_BONUS: u64 = 5; // 5
pub const MIN_HEALTH_FACTOR: u64 = 100; // 100%