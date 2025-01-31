use anchor_lang::prelude::*;

// Information for every NFT listed
#[account]
pub struct ListingAccount {
    pub creater: Pubkey,
    pub nft_mint: Pubkey,
    pub nft_price: u16,
    pub listing_bump: u8,
}
