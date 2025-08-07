pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8Z935UApS1fPcyhTQ42KzWvYW83j3wrCEmJCiwVz7EVC");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMarketplace>, params: InitializeParams) -> Result<()> {
        ctx.accounts
            .handle(params.name, params.fee_bps, &ctx.bumps)?;

        Ok(())
    }

    pub fn create_nft(ctx: Context<CreateNFT>, params: CreateNFTParams) -> Result<()> {
        ctx.accounts.create_nft(params)
    }

    pub fn modify_nft(ctx: Context<UpdateNFTMetadata>, params: UpdateNFTParams) -> Result<()> {
        ctx.accounts.update_metadata(params)
    }

    pub fn list_nft(ctx: Context<ListNFT>, params: InitializeListingParams) -> Result<()> {
        ctx.accounts.initialize_listing(params, &ctx.bumps)?;
        ctx.accounts.list_nft()?;
        Ok(())
    }

    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.make_payment()?;
        ctx.accounts.transfer_nft()?;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq)]
pub struct InitializeParams {
    name: String,
    fee_bps: u16,
}
