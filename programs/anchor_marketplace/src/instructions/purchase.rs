use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use mpl_core::types::PermanentBurnDelegate;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::TransferV1CpiBuilder,
};
use mpl_core::{BasePlugin, PermanentBurnDelegatePlugin};

use crate::error::MarketplaceError;
use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    /// CHECK: The seller account that will receive payment
    #[account(
        mut,
        constraint = seller.key() == listing.seller @ MarketplaceError::SellerMismatch
    )]
    pub seller: SystemAccount<'info>,

    /// The MPL Core asset being purchased
    #[account(
        mut,
        constraint = asset.key() == listing.mint @ MarketplaceError::AssetMismatch,
    )]
    pub asset: Account<'info, BaseAssetV1>,

    /// The collection that this asset belongs to
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    /// CHECK: The escrow account that currently holds the asset
    #[account(
        mut,
        seeds = [b"escrow", listing.key().as_ref()],
        bump,
        constraint = asset.owner == escrow.key() @ MarketplaceError::AssetNotInEscrow,
    )]
    pub escrow: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"listing", marketplace.key().as_ref(), asset.key().as_ref()],
        bump = listing.bump,
        constraint = listing.is_active @ MarketplaceError::ListingNotActive,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"marketplace", seller.key().as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    /// CHECK: MPL Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn make_payment(&mut self) -> Result<()> {
        let token_price = self.listing.get_price_by_token_id();
        let marketplace_fee = self.marketplace.fee_bps as u64;

        // Calculate fee to transfer to marketplace treasury
        let amount_to_transfer_as_fee = token_price
            .checked_mul(marketplace_fee)
            .and_then(|mul_result| mul_result.checked_div(10_000))
            .ok_or_else(|| error!(MarketplaceError::MathOverflowError))?;

        // Calculate remaining amount to transfer to seller
        let amount_to_transfer_to_seller = token_price
            .checked_sub(amount_to_transfer_as_fee)
            .ok_or_else(|| error!(MarketplaceError::MathOverflowError))?;

        // Transfer fee to treasury
        let cpi_account_fee_ix = Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        transfer(
            CpiContext::new(self.system_program.to_account_info(), cpi_account_fee_ix),
            amount_to_transfer_as_fee,
        )?;

        // Transfer remaining amount to seller
        let cpi_account_amount_seller_ix = Transfer {
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                cpi_account_amount_seller_ix,
            ),
            amount_to_transfer_to_seller,
        )?;

        Ok(())
    }

    pub fn transfer_nft(&mut self) -> Result<()> {
        // use escrow seeds since escrow is now the new owner

        let listing_key = &self.listing.key();
        let signers_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            &listing_key.as_ref(),
            &[self.listing.escrow_bump],
        ]];

        // Transfer the MPL Core asset from escrow to buyer
        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .payer(&self.buyer.to_account_info())
            .authority(Some(&self.escrow.to_account_info()))
            .new_owner(&self.buyer.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke_signed(signers_seeds)?;

        Ok(())
    }
}
