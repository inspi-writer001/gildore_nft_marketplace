use anchor_lang::prelude::*;

use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::{
        AddPluginV1CpiBuilder, ApprovePluginAuthorityV1CpiBuilder, TransferV1CpiBuilder,
    },
    types::{
        BurnDelegate, FreezeDelegate, Plugin, PluginAuthority, TransferDelegate, UpdateAuthority,
    },
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
        mut,
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
    pub fn initialize_listing(
        &mut self,
        params: InitializeListingParams,
        bumps: &ListNFTBumps,
    ) -> Result<()> {
        self.listing.set_inner(Listing {
            seller: self.seller.key(),
            mint: self.asset.key(),
            price: params.price,
            bump: bumps.listing,
            escrow_bump: bumps.escrow,
            is_active: true,
            token_id: params.token_id,
        });
        Ok(())
    }

    pub fn list_nft(&mut self) -> Result<()> {
        // Transfer the MPL Core asset to the escrow account
        // note that I'm passing authority on all CPIBuilders because The owner is the authority (owner) of the asset, it's not delegated so owner can sign on behalf of this tx and as the authority of the asset
        // add TransferDelegate Plugin
        AddPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .authority(Some(&self.seller.to_account_info()))
            .asset(&self.asset.to_account_info())
            .payer(&self.seller.to_account_info())
            .plugin(Plugin::TransferDelegate(TransferDelegate {}))
            .system_program(&self.system_program.to_account_info())
            .invoke()?;

        // Approve TransferDelegate Plugin to listing
        ApprovePluginAuthorityV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .new_authority(PluginAuthority::Address {
                address: self.listing.key(),
            })
            .authority(Some(&self.seller.to_account_info()))
            .asset(&self.asset.to_account_info())
            .payer(&self.seller.to_account_info())
            .plugin_type(mpl_core::types::PluginType::TransferDelegate)
            .system_program(&self.system_program.to_account_info())
            .invoke()?;

        // add BurnDelegate Plugin
        AddPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .authority(Some(&self.seller.to_account_info()))
            .asset(&self.asset.to_account_info())
            .payer(&self.seller.to_account_info())
            .plugin(Plugin::BurnDelegate(BurnDelegate {}))
            .system_program(&self.system_program.to_account_info())
            .invoke()?;

        // approve BurnDelegate Plugin to listing
        ApprovePluginAuthorityV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .new_authority(PluginAuthority::Address {
                address: self.listing.key(),
            })
            .authority(Some(&self.seller.to_account_info()))
            .asset(&self.asset.to_account_info())
            .payer(&self.seller.to_account_info())
            .plugin_type(mpl_core::types::PluginType::BurnDelegate)
            .system_program(&self.system_program.to_account_info())
            .invoke()?;

        // add Freeze Delegate
        AddPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .authority(Some(&self.seller.to_account_info()))
            .asset(&self.asset.to_account_info())
            .payer(&self.seller.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false }))
            .system_program(&self.system_program.to_account_info())
            .invoke()?;

        // Approve Freeze Delegate to listing
        ApprovePluginAuthorityV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .new_authority(PluginAuthority::Address {
                address: self.listing.key(),
            })
            .authority(Some(&self.seller.to_account_info()))
            .asset(&self.asset.to_account_info())
            .payer(&self.seller.to_account_info())
            .plugin_type(mpl_core::types::PluginType::FreezeDelegate)
            .system_program(&self.system_program.to_account_info())
            .invoke()?;

        // Transfer token to Escrow -> Now Listing can make Tx on behalf of Escrow for the Asset
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

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeListingParams {
    pub price: u64,
    pub token_id: u16,
}
