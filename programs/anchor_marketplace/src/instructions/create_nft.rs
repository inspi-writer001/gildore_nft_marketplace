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
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    #[account(
        mut,
        seeds = [b"marketplace", creator.key().as_ref()],
        bump = marketplace.bump,
        constraint = marketplace.admin == creator.key() @ MarketplaceError::UnauthorizedCreator
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// CHECK: MPL Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateNFT<'info> {
    pub fn create_nft(&mut self, params: CreateNFTParams) -> Result<()> {
        // Store AccountInfo values in variables to extend their lifetime
        let mpl_core_program_info = self.mpl_core_program.to_account_info();
        let asset_info = self.asset.to_account_info();
        let creator_info = self.creator.to_account_info();
        let system_program_info = self.system_program.to_account_info();
        let collection_info = self.collection.as_ref().map(|c| c.to_account_info());
        // Create the MPL Core asset
        let mut builder = CreateV1CpiBuilder::new(&mpl_core_program_info);

        builder
            .asset(&asset_info)
            .payer(&creator_info)
            .owner(Some(&creator_info))
            .update_authority(Some(&creator_info))
            .system_program(&system_program_info)
            .name(params.name)
            .collection(collection_info.as_ref())
            .uri(params.uri);

        // // Handle optional collection
        // match &self.collection {
        //     Some(collection_exists) => {
        //         let collection_info: AccountInfo<'_> = collection_exists.to_account_info();
        //         builder.collection(Some(&collection_info));
        //     }
        //     None => {
        //         builder.collection(None);
        //     }
        // };

        builder.invoke()?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CreateNFTParams {
    pub name: String,
    pub uri: String,
}
