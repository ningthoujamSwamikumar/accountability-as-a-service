use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::error::AaaSErrorCode;
use crate::{constants::MAX_MEMBER, Pool};

pub fn join_pool_handler(ctx: Context<JoinPool>, _pool_id: u64) -> Result<()> {
    let signer_key = ctx.accounts.joiner.key;
    let pool = &ctx.accounts.pool;

    require!(pool.members.len() < MAX_MEMBER, AaaSErrorCode::MaxMember);
    require!(
        pool.start_time > Clock::get()?.unix_timestamp as u64,
        AaaSErrorCode::ChallengeStarted
    );

    send_token_to_vault(
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

fn send_token_to_vault<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u16,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        authority: authority.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
    };
    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);
    transfer_checked(cpi_context, *amount as u64, mint.decimals)
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
