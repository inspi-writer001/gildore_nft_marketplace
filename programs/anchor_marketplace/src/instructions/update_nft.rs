use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::{CreateV1CpiBuilder, UpdateV1CpiBuilder},
    types::{DataState, Plugin, PluginAuthority, UpdateAuthority},
};

use crate::MarketplaceError;

#[derive(Accounts)]
pub struct UpdateNFTMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The MPL Core asset (NFT) to be updated
    #[account(
        mut,
        constraint = asset.update_authority == UpdateAuthority::Address(authority.key()) @ MarketplaceError::NotUpdateAuthority,
    )]
    pub asset: Account<'info, BaseAssetV1>,

    /// The collection that this asset belongs to (optional)
    pub collection: Option<Account<'info, BaseCollectionV1>>,

    /// CHECK: MPL Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateNFTMetadata<'info> {
    pub fn update_metadata(&mut self, params: UpdateNFTParams) -> Result<()> {
        let mpl_program = self.mpl_core_program.to_account_info();
        let authority = self.authority.to_account_info();
        let asset = self.asset.to_account_info();
        let system_prog = self.system_program.to_account_info();

        let mut builder = UpdateV1CpiBuilder::new(&mpl_program);

        builder
            .asset(&asset)
            .payer(&authority)
            .authority(Some(&authority))
            .system_program(&system_prog)
            .new_name(match params.name {
                Some(name_provided) => name_provided,
                None => String::new(),
            })
            .new_uri(match params.uri {
                Some(uri_provided) => uri_provided,
                None => String::new(),
            });

        builder.invoke()?;

        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UpdateNFTParams {
    pub name: Option<String>,
    pub uri: Option<String>,
}
