use anchor_lang::prelude::*;

// This is like a brain 🧠 for all NFTs which are going to sell.

#[account]
pub struct Marketplace {
    pub admin: Pubkey, // The person who created this marketplace
    pub fees: u16,
    pub marketplace_bump: u8,
    pub treasury_bump: u8, // My Earings will be stored inside of this system_account
    pub reward_bum: u8,    // The tokens for rewarding the creaters
    pub name: String,
}

impl Marketplace {
    pub const LEN: usize = 8 + 32 + 2 + 1 + 1 + (32 + 4);
}
