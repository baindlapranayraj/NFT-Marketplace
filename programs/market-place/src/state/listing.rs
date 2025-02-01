use anchor_lang::prelude::*;

// Information for every NFT listed
#[account]
pub struct ListingAccount {
    pub creater: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_price: u16,
    pub listing_bump: u8,
}

impl ListingAccount {
    pub const LIST_SIZE: usize = 8 + 32 + 32 + 2 + 1;
}
