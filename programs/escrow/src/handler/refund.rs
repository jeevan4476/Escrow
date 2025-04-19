use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use super::shared::{close_token_account, transfer_tokens};
use crate::state::Offer;

#[derive(Accounts)]
pub struct RefundOffer<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,

    pub token_mint_a : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a : Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(
        mut,
        close=maker ,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Box<Account<'info,Offer>>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault : Box<InterfaceAccount<'info,TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}

// Handle the refund offer instruction by:
// 1. Returning the tokens from the vault to the maker's account
// 2. Closing the vault and returning the rent to the maker