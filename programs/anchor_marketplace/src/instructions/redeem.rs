use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::{BurnV1CpiBuilder, TransferV1CpiBuilder},
    types::UpdateAuthority,
};

use crate::{Listing, Marketplace, MarketplaceError};

#[derive(Accounts)]
pub struct RedeemNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = asset.key() == listing.mint @ MarketplaceError::AssetMismatch,
    )]
    pub asset: Account<'info, BaseAssetV1>,

    #[account(
        mut,
        seeds = [b"listing", marketplace.key().as_ref(), asset.key().as_ref()],
        bump = listing.bump,
        close = owner,
        constraint = listing.is_active @ MarketplaceError::ListingNotActive,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"marketplace", listing.seller.key().as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: The escrow account that currently holds the asset
    #[account(
        mut,
        seeds = [b"escrow", listing.key().as_ref()],
        bump,
        constraint = asset.update_authority == UpdateAuthority::Address(escrow.key()) @ MarketplaceError::AssetNotInEscrow,
    )]
    pub escrow: UncheckedAccount<'info>,

    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RedeemNFT<'info> {
    pub fn redeem_nft(&mut self) -> Result<()> {
        let listing = &self.listing.key();
        let signers_seeds: &[&[&[u8]]] = &[&[b"escrow", listing.as_ref()]];

        BurnV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(None)
            .payer(&self.owner.to_account_info())
            .authority(Some(&self.escrow.to_account_info()))
            .invoke_signed(&signers_seeds)?;

        Ok(())
    }
}
