use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata}, // new
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
// use mpl_token_metadata::{
//     pda::{find_master_edition_account, find_metadata_account}, // new
// };

use mpl_token_metadata::accounts::{ MasterEdition, Metadata as MetadataAccount };

/// CHECK: metadata address check
    #[account(
        mut,
        address = MetadataAccount::find_pda(&mint.key()).0,
    )]

/// CHECK: master edition address address check
    #[account(
        mut,
        address = MasterEdition::find_pda(&mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,

declare_id!("GpCPiNi4FhKQcvzUPNKNoeuXvgNiK97LWkzyztcRnBDS");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn init_nft(ctx: Context<InitNFT>) -> Result<()> {
        // create mint account
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.signer.to_account_info(),
            },
        );

        mint_to(cpi_context, 1)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing in this account ourselves
    #[account(mut, signer)]
    pub signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    /// CHECK - address
    #[account(
        mut,
        address=find_metadata_account(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>, // new
    /// CHECK: address
    #[account(
        mut,
        address=find_master_edition_account(&mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>, // new

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>, // new
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

let metadata_seeds = &[PREFIX.as_bytes(), program_id.as_ref(), mint_pubkey.as_ref()];
let (pubkey, _) = Pubkey::find_program_address(metadata_seeds, &ID);
