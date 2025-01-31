use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::init::InitMarketplace;
pub use state::marketplace;

declare_id!("41NF3e7ThxSRwgpYWvpRfDrmvza9gXcPTyqEqQ3ZKQkJ");

#[program]
pub mod market_place {

    use super::*;

    pub fn initialize(ctx: Context<InitMarketplace>, name: String, fee: u16) -> Result<()> {
        let bumps = ctx.bumps;
        ctx.accounts.initialize_marketplace(name, fee, bumps)?;
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
