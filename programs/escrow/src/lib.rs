#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod handler;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use handler::*;
pub use state::*;

declare_id!("F8CYg2tDt59xQn5K9kbL3qCGbgo8YCvf3iQBvZcJwRuJ");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        handler::make_offer::make_offer(context, id, token_a_offered_amount, token_b_wanted_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        handler::take_offer::take_offer(context)
    }

    pub fn refund_offer(context: Context<RefundOffer>) -> Result<()> {
        handler::refund::refund_offer(context)
    }
}
