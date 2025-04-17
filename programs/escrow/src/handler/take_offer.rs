use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Offer;

use super::shared::{close_token_account, transfer_tokens};

#[derive(Accounts)]
pub struct TakeOffer<'info>{
    #[account(mut)]
    pub taker : Signer<'info>,

    #[account(mut)]
    pub maker : SystemAccount<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,

    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,


    #[account(
        mut,
        close=maker,
        has_one=maker,
        has_one=token_mint_b,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump=offer.bump
    )]
    pub offer: Box<Account<'info, Offer>>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


pub fn take_offer(context:Context<TakeOffer> )->Result<()>{
    transfer_tokens(
        &context.accounts.taker_token_account_b, 
        &context.accounts.taker_token_account_a,
            &context.accounts.offer.token_b_wanted_amount, 
        &context.accounts.token_mint_b, 
        &context.accounts.taker.to_account_info(), 
        owning_pda_seeds,
    None)?;

    let offer_account_seeds = &[
        b"offer",
        context.accounts.maker.to_account_info().key.as_ref(),
        &context.accounts.offer.id.to_le_bytes()[..],
        &[context.accounts.offer.bump],
    ];

    let signer_seeds = Some(&offer_account_seeds[..]);

    transfer_tokens(
        &context.accounts.taker_token_account_b,
        &context.accounts.maker_token_account_b,
        &context.accounts.offer.token_b_wanted_amount,
        &context.accounts.token_mint_b,
        &context.accounts.taker.to_account_info(),
        &context.accounts.token_program,
        signer_seeds);

        close_token_account(
            &context.accounts.vault,
            &context.accounts.taker.to_account_info(),
            &context.accounts.offer.to_account_info(),
            &context.accounts.token_program,
            signers_seeds,
        )
}