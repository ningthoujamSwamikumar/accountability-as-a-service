use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{constants::{ANCHOR_DISCRIMINATOR, MAX_PROOF_TYPES}, error::AaaSErrorCode, state::Pool};

pub fn handler(
    ctx: Context<CreatePool>,
    pool_id: u64,
    start_time: u64,
    end_time: u64,
    entry_fee: u16,
    accepted_proofs: Vec<String>,
    goal: String,
) -> Result<()> {
    require!(
        accepted_proofs.len() >= MAX_PROOF_TYPES,
        AaaSErrorCode::MaxProofLimit
    );
    require!(start_time < end_time, AaaSErrorCode::TimeOrder);

    *ctx.accounts.pool = Pool {
        pool_id,
        start_time,
        end_time,
        entry_fee,
        accepted_proofs,
        goal,
        members: Vec::new()
    };
    Ok(())
}

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
