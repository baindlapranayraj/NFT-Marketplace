use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{
    constant::{LIST_NFT, MARKETPLACE, REWARD, TRESURY},
    state::{listing::ListingAccount, marketplace::Marketplace},
};

// +++++++++++++ Accounts +++++++++++++
//  - creater_acc <>
//  - buyer_acc <>
//  - buyer_nft_acc <>
//  - vault_acc <>
//  - market_place_acc <>
//  - list_acc <>
//  - mint_acc <>
// - treasury_acc <>
// - reward_acc <>

#[derive(Accounts)]
pub struct PurchaseNFT<'info> {
    #[account(mut)]
    pub creater: SystemAccount<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [MARKETPLACE,market_place.name.as_bytes()],
        bump
    )]
    pub market_place: Account<'info, Marketplace>,

    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = creater,
        seeds = [LIST_NFT,market_place.key().as_ref(),nft_mint.key().as_ref()],
        bump = listing_account.listing_bump
    )]
    pub listing_account: Account<'info, ListingAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing_account
    )]
    pub nft_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [TRESURY,market_place.key().as_ref()],
        bump = market_place.treasury_bump
    )]
    pub treasury_account: SystemAccount<'info>,

    #[account(
        mut,
        seeds=[REWARD,market_place.key().as_ref()],
        bump = market_place.reward_bum,
        mint::decimals = 6,
        mint::authority = market_place,
    )]
    pub reward_account: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> PurchaseNFT<'info> {
    // pay sol for tresury and creater from taker
    pub fn deposite_amount(&mut self) -> Result<()> {
        // the platform fee for this NFT
        let platform_fee = (self.market_place.fees as u64)
            .checked_mul(self.listing_account.nft_price as u64)
            .ok_or(ProgramError::InvalidArgument)?
            / 10000;

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);
        transfer(cpi_ctx, platform_fee)?; // transfer from buyer to tresury

        let cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.creater.to_account_info(),
        };

        let nft_price = self.listing_account.nft_price as u64;

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, nft_price)?; // transfer from buyer to creater

        Ok(())
    }

    pub fn transfer_nft(&mut self) -> Result<()> {
        let seeds = &[
            LIST_NFT,
            self.market_place.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.listing_account.listing_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            TransferChecked {
                authority: self.listing_account.to_account_info(),
                from: self.nft_vault.to_account_info(),
                to: self.buyer.to_account_info(),
                mint: self.nft_mint.to_account_info(),
            },
            signer_seeds,
        );

        transfer_checked(ctx, 1, self.nft_mint.decimals);
        Ok(())
    }

    pub fn close_accounts(&mut self) -> Result<()> {
        let seeds = &[
            LIST_NFT,
            self.market_place.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.nft_vault.to_account_info(),
                destination: self.creater.to_account_info(),
                authority: self.listing_account.to_account_info(),
            },
            signer_seeds,
        );

        close_account(ctx);

        Ok(())
    }
}
