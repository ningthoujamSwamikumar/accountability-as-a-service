use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::MAX_MEMBER, error::AaaSErrorCode, shared, Pool};

pub fn join_pool_handler(ctx: Context<JoinPool>, _pool_id: u64) -> Result<()> {
    let signer_key = ctx.accounts.joiner.key;
    let pool = &ctx.accounts.pool;

    require!(pool.members.len() < MAX_MEMBER, AaaSErrorCode::MaxMember);
    require!(
        pool.start_time > Clock::get()?.unix_timestamp as u64,
        AaaSErrorCode::ChallengeStarted
    );

    shared::transfer_token(
        &ctx.accounts.fee_token_account,
        &ctx.accounts.vault,
        &pool.entry_fee,
        &ctx.accounts.fee_token_mint,
        &ctx.accounts.joiner,
        &ctx.accounts.fee_token_program,
    )?;

    let pool = &mut ctx.accounts.pool;
    pool.members.push(signer_key.to_string());

    Ok(())
}

#[derive(Accounts)]
#[instruction(pool_id: u64)]
pub struct JoinPool<'info> {
    #[account(mut)]
    pub joiner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"pool", pool_id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub fee_token_program: Interface<'info, TokenInterface>,

    #[account(mint::token_program = fee_token_program)]
    pub fee_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub fee_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = fee_token_mint,
        associated_token::authority = pool,
        associated_token::token_program = fee_token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
