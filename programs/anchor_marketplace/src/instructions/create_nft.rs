use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::{CreateV1CpiBuilder, UpdateV1CpiBuilder},
    types::{DataState, Plugin, PluginAuthority, UpdateAuthority},
};

pub use crate::error::MarketplaceError;
use crate::Marketplace;

#[derive(Accounts)]
#[instruction(params: CreateNFTParams)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// The MPL Core asset (NFT) to be created
    /// CHECK: This account will be created by MPL Core
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    /// The collection that this asset will belong to
    #[account(
        constraint = collection.update_authority == creator.key() @ MarketplaceError::NotUpdateAuthority,
    )]
    pub collection: Account<'info, BaseCollectionV1>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: MPL Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateNFT<'info> {
    pub fn create_nft(&mut self, params: CreateNFTParams) -> Result<()> {
        // Create the MPL Core asset
        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.creator.to_account_info())
            .owner(Some(&self.creator.to_account_info()))
            .update_authority(Some(&self.creator.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(params.name)
            .uri(params.uri)
            .invoke()?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CreateNFTParams {
    pub name: String,
    pub uri: String,
}
