use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{Pool, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR + Pool::INIT_SPACE,
        seeds = [b"pool", pool_id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mint::token_program=fee_token_program)]
    pub fee_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = fee_token_mint,
        associated_token::authority = pool,
        associated_token::token_program = fee_token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub fee_token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
