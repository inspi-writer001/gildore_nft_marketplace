use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name cannot be undefined")]
    UndefinedName,
    #[msg("Name cannot be more than 32 characters long")]
    NameTooLong,
    #[msg("Error  occured performing arithmetic probable overflow")]
    MathOverflowError,
    #[msg("Fee basis points cannot exceed 10000 (100%)")]
    InvalidFeeBps,
    #[msg("Seller is not the owner of the asset")]
    NotAssetOwner,
    #[msg("Seller is not the update authority of the asset")]
    NotUpdateAuthority,
    #[msg("Asset does not belong to the specified collection")]
    CollectionMismatch,
    #[msg("You have a wrong seller")]
    SellerMismatch,
    #[msg("You have a wrong asset")]
    AssetMismatch,
    #[msg("This Asset is not in Escrow")]
    AssetNotInEscrow,
    #[msg("Asset Listing is not active")]
    ListingNotActive,
}
