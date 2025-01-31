use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::errors::MarketPlaceErrors;
use crate::state::marketplace::Marketplace;

// Our program controlls the marketplace pda, treasury pda and reward pda.

#[derive(Accounts)]
#[instruction(name:String)]
pub struct InitMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space =  Marketplace::LEN,
        seeds = [b"market_place",name.as_bytes()],
        bump
    )]
    pub market_place_account: Account<'info, Marketplace>, // Our PDA

    #[account(
        seeds = [b"treasury_place",market_place_account.key().as_ref()],
        bump
    )]
    pub treasury_account: SystemAccount<'info>, // Admins treasury amount will be collected hear.

    #[account(
        init,
        payer = admin,
        seeds = [b"reward_account",market_place_account.key().as_ref()],
        bump,
        mint::authority = market_place_account, // This acc is contolled by our program.
        mint::decimals = 6
    )]
    pub reward_account: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitMarketplace<'info> {
    pub fn initialize_marketplace(
        &mut self,
        name: String,
        fee: u16,
        bumps: InitMarketplaceBumps,
    ) -> Result<()> {
        require!(
            name.len() > 1 && name.len() <= 32,
            MarketPlaceErrors::TooLongName
        );

        self.market_place_account.set_inner(Marketplace {
            admin: self.admin.to_account_info().key(),
            fees: fee,
            marketplace_bump: bumps.market_place_account,
            treasury_bump: bumps.treasury_account,
            reward_bum: bumps.reward_account,
            name,
        });

        Ok(())
    }
}
