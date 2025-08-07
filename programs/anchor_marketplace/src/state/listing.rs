use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
    pub token_id: u16,
    pub is_active: bool,
}
impl Listing {
    pub fn get_price_by_token_id(&self) -> u64 {
        // TODO implement price fetching from oracle based on nft type
        return self.price;
    }
}
