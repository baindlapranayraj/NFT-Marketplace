use anchor_lang::prelude::*;

declare_id!("41NF3e7ThxSRwgpYWvpRfDrmvza9gXcPTyqEqQ3ZKQkJ");

pub mod constant;
pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::{
    delist::Delist, init::InitMarketplace, list::List, purchase::PurchaseNFT,
};

#[program]
pub mod market_place {

    use super::*;

    pub fn initialize(ctx: Context<InitMarketplace>, name: String, fee: u16) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.initialize_marketplace(name, fee, bumps)?;
        Ok(())
    }

    pub fn deposite_nft(ctx: Context<List>, price: u16) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.initialize_list(price, bumps)?;
        ctx.accounts.deposite_nft()?;
        Ok(())
    }

    pub fn withdraw_nft(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist_nft()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }

    pub fn purchase_nft(ctx: Context<PurchaseNFT>) -> Result<()> {
        ctx.accounts.deposite_amount()?;
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_accounts()?;
        Ok(())
    }
}

// NFT Marketplace
// - list:- creaters will come and keep there NFTs hear to sell
// - delist:- creater can remove there NFTs from this Markeyplace(His rent will be given back)
// - buy:- people will come and buy the NFTs and the amount is sended to creater and there will 10% cut for me 🙃

// ++++++++++++++++++++++ Key Learnings  ++++++++++++++++++++++
// - If u want to control systemAccount or Token Accounts or anyother accounts by our program, then it should be connected with
//   our program PDA.
