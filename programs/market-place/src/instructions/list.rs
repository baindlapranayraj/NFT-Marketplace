use crate::{
    constant::*,
    state::{listing::ListingAccount, marketplace::Marketplace},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

// ++++++++++++++++ Accounts Requierd ++++++++++++++++
// - createer account <>
// - marketplace account <>
// - list account <>
// - creater_nft_mint <>
// - creater_nft_token_account <>
// - nft_vault <>

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub creater: Signer<'info>,

    pub creater_mint: InterfaceAccount<'info, Mint>, // Madlad NFT Mint Account

    #[account(
        mut,
        seeds = [MARKETPLACE,marketplace.name.as_bytes()],
        bump = marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = creater,
        space = ListingAccount::LIST_SIZE,
        seeds = [
            LIST_NFT,
            marketplace.key().to_bytes().as_ref(),
            creater_mint.key().to_bytes().as_ref()
        ],
        bump
    )]
    pub list_account: Account<'info, ListingAccount>,

    #[account(
        mut,
        associated_token::mint = creater_mint,
        associated_token::authority = creater
    )]
    pub creater_nft_account: InterfaceAccount<'info, TokenAccount>, // Madlad NFT Token Account

    #[account(
        init,
        payer = creater,
        associated_token::mint = creater_mint,
        associated_token::authority = list_account,
    )]
    pub nft_vault_account: InterfaceAccount<'info, TokenAccount>,

    // NFT Validation accounts
    pub collection_mint: InterfaceAccount<'info, Mint>, // to verify NFT belongs to Madlad collection

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key.as_ref(),
            creater_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key,
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(), // chekcing the collection validity
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key.as_ref(),
            creater_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    // Program Accounts
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> List<'info> {
    pub fn initialize_list(&mut self, price: u16, bumps: ListBumps) -> Result<()> {
        self.list_account.set_inner(ListingAccount {
            creater: self.creater.key(),
            nft_mint: self.creater_mint.key(),
            nft_price: price,
            listing_bump: bumps.list_account,
        });
        Ok(())
    }

    pub fn deposite_nft(&mut self) -> Result<()> {
        // Depositing the NFT from user wallet to program contolled vault

        let cpi_program = self.token_program.to_account_info();
        let tranfer_accounts = TransferChecked {
            authority: self.creater.to_account_info(),
            mint: self.creater_mint.to_account_info(),
            from: self.creater_nft_account.to_account_info(),
            to: self.nft_vault_account.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, tranfer_accounts);

        transfer_checked(cpi_context, 1, self.creater_mint.decimals)?;

        Ok(())
    }
}
