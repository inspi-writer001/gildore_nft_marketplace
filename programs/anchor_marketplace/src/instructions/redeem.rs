use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use mpl_core::{
    accounts::BaseAssetV1,
    instructions::{BurnV1CpiBuilder, UpdatePluginV1CpiBuilder},
    types::{FreezeDelegate, Plugin, UpdateAuthority},
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

    pub seller: SystemAccount<'info>,

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

    #[account(
        mut,
        seeds = [b"listing", marketplace.key().as_ref(), asset.key().as_ref()],
        bump = listing.bump,
        close = owner,
        constraint = listing.is_active @ MarketplaceError::ListingNotActive,
        constraint = seller.key() == listing.seller.key() @ MarketplaceError::NotAssetOwner
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: The escrow account that currently holds the asset
    #[account(
        mut,
        seeds = [b"escrow", listing.key().as_ref()],
        bump = listing.escrow_bump,
        // constraint = asset.update_authority == UpdateAuthority::Address(escrow.key()) @ MarketplaceError::AssetNotInEscrow,
    )]
    pub escrow: UncheckedAccount<'info>,

    /// CHECK: MPL Program ID
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RedeemNFT<'info> {
    pub fn redeem_nft(&mut self) -> Result<()> {
        let token_price = self.listing.get_price_by_token_id();
        let marketplace_fee = self.marketplace.fee_bps as u64;

        // Pay half purchase fee to redeem physically
        let amount_to_transfer_as_fee = token_price
            .checked_mul(marketplace_fee)
            .and_then(|mul_result| mul_result.checked_div(10_000))
            .ok_or_else(|| error!(MarketplaceError::MathOverflowError))?;

        let half_amount = amount_to_transfer_as_fee
            .checked_div(2)
            .ok_or_else(|| error!(MarketplaceError::MathOverflowError))?;

        // Transfer fee to treasury
        let cpi_account_fee_ix = Transfer {
            from: self.owner.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        transfer(
            CpiContext::new(self.system_program.to_account_info(), cpi_account_fee_ix),
            half_amount,
        )?;
        let marketplace_key = &self.marketplace.key();
        let asset_key = &self.asset.key();

        let signers_seeds: &[&[&[u8]]] = &[&[
            b"listing",
            &marketplace_key.as_ref(),
            &asset_key.as_ref(),
            &[self.listing.bump],
        ]];

        // Thaw asset - unfreeze asset
        // note that I'm not passing authority because Listing isnt the authority (owner) of the asset, it was delegated to listing so listing can sign on behalf of this tx
        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .payer(&self.owner.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false }))
            .system_program(&self.system_program.to_account_info())
            .invoke_signed(signers_seeds)?;

        // Burning the asset
        // note that I'm not passing authority because Listing isnt the authority (owner) of the asset, it was delegated to listing so listing can sign on behalf of this tx
        BurnV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(None)
            .payer(&self.owner.to_account_info())
            .system_program(Some(&self.system_program.to_account_info()))
            .invoke_signed(&signers_seeds)?;

        Ok(())
    }
}
