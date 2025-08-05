use anchor_lang::{prelude::*, solana_program::sysvar::rent, system_program::{self, Transfer}};
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{error::MarketplaceError, Marketplace};

#[derive(Accounts)]

pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account( 
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
   
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", admin.key().as_ref()],
        bump,
        space = 8 + Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,

    // #[account(
    //     init,
    //     payer = admin,
    //     mint::decimals = 6,
    //     mint::authority = marketplace,
    //     seeds = [b"rewards", marketplace.key().as_ref()],
    //     bump
    // )]
    // pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeMarketplace<'info> {
    pub fn handle(&mut self, name: String, fee_bps: u16, bumps: &InitializeMarketplaceBumps) -> Result<()>{

        require!(name.len() < 4 + 32, MarketplaceError::NameTooLong);
        require!(name.len() > 0, MarketplaceError::UndefinedName);
        require!(
            fee_bps <= 10000, 
            MarketplaceError::InvalidFeeBps
        );
        self.marketplace.set_inner(Marketplace { admin: self.admin.key(), treasury_bump: bumps.treasury, bump: bumps.marketplace, fee_bps, name });

        // create treasury account by transfering minimum amount for rent
        let amount_for_rent =  rent::Rent::get()?.minimum_balance(self.treasury.to_account_info().data_len());
        let transfer_accounts = Transfer {
            from: self.admin.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let cpi_context = CpiContext::new(self.system_program.to_account_info(), transfer_accounts);

        system_program::transfer(cpi_context, amount_for_rent)?;
        Ok(())
    }
}
