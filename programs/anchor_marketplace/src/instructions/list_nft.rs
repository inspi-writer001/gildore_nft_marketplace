use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::TransferV1CpiBuilder,
    types::UpdateAuthority,
};

pub use crate::error::MarketplaceError;
use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct ListNFT<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    /// The MPL Core asset (NFT) to be listed
    #[account(
        mut,
        constraint = asset.owner == seller.key() @ MarketplaceError::NotAssetOwner,
        constraint = asset.update_authority == UpdateAuthority::Address(seller.key()) @ MarketplaceError::NotUpdateAuthority,
    )]
    pub asset: Account<'info, BaseAssetV1>,

    /// The collection that this asset belongs to
    #[account(
        constraint = asset.update_authority == UpdateAuthority::Address(collection.update_authority) @ MarketplaceError::CollectionMismatch,
    )]
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    #[account(
        seeds = [b"marketplace", seller.key().as_ref()],
        bump = marketplace.bump,
        constraint = seller.key() == marketplace.admin @ MarketplaceError::UnauthorizedCreator
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init,
        payer = seller,
        seeds = [b"listing", marketplace.key().as_ref(), asset.key().as_ref()],
        bump,
        space = 8 + Listing::INIT_SPACE
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: This is the PDA that will receive the asset
    #[account(
        seeds = [b"escrow", listing.key().as_ref()],
        bump,
        
    )]
    pub escrow: UncheckedAccount<'info>,

    /// CHECK: MPL Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ListNFT<'info> {
    pub fn initialize_listing(&mut self, price: u64, bumps: &ListNFTBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            seller: self.seller.key(),
            mint: self.asset.key(), 
            price,
            bump: bumps.listing,
            is_active: true,
        });
        Ok(())
    }

    pub fn list_nft(&mut self) -> Result<()> {
        // Transfer the MPL Core asset to the escrow account
        TransferV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .payer(&self.seller.to_account_info())
            .authority(Some(&self.seller.to_account_info()))
            .new_owner(&self.escrow.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke()?;

        Ok(())
    }
}

// // Custom program wrapper for MPL Core
// #[derive(Clone)]
// pub struct MplCore;

// impl anchor_lang::Id for MplCore {
//     fn id() -> Pubkey {
//         MPL_CORE_PROGRAM_ID.try_into().unwrap()
//     }
// }
