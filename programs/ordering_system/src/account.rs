use anchor_lang::prelude::*;

use crate::constants::*;
#[account]
pub struct Config {
  pub royalties: [Royalty; MAX_ROYALTY_COUNT as usize],
  pub count: u32
}

#[account]
pub struct Order {
  pub order_id: u64,
  pub amount: u64,
  pub user: Pubkey,
  pub status: u32 // 0: created, 1: rejected, 2: accepted, 3: achieved, 4, canceled: 5, forced: 6
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, Default)]
pub struct Royalty {
  pub wallet: Pubkey,
  pub percent: u64
}
