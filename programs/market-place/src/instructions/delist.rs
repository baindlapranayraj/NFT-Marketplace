use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{
    constant::*,
    state::{listing::ListingAccount, marketplace::Marketplace},
};

// ============ Accounts ============
// - creater <>
// - vault
// - nft_acc <>
// - creater_mint_acco <>
// - list_acc <>
// - market_place account <>

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub creater: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE,market_place.name.as_bytes().as_ref()],
        bump = market_place.marketplace_bump
    )]
    pub market_place: Account<'info, Marketplace>,

    pub creater_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = creater_mint,
        associated_token::authority = creater
    )]
    pub creater_nft: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = creater_mint,
        associated_token::authority = list_account
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds =[
            LIST_NFT,
            market_place.key().as_ref(),
            creater_mint.key().as_ref()
        ],
        bump = list_account.listing_bump,
        close = creater
    )]
    pub list_account: Account<'info, ListingAccount>,

    // program accounts
    pub system_account: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_program: Program<'info, AssociatedToken>,
}

impl<'info> Delist<'info> {
    pub fn delist_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            authority: self.list_account.to_account_info(),
            mint: self.creater_mint.to_account_info(),
            from: self.nft_vault.to_account_info(),
            to: self.creater_nft.to_account_info(),
        };

        let seeds = &[
            LIST_NFT,
            self.market_place.to_account_info().key.as_ref(),
            self.creater_mint.to_account_info().key.as_ref(),
        ];

        let signer_seeds = &[&seeds[..]];
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_context, 1, self.creater_mint.decimals)?;
        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.nft_vault.to_account_info(),
            authority: self.list_account.to_account_info(),
            destination: self.creater.to_account_info(),
        };

        let seeds = &[
            LIST_NFT,
            self.market_place.to_account_info().key.as_ref(),
            self.creater_mint.to_account_info().key.as_ref(),
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        close_account(ctx)?;
        Ok(())
    }
}
