use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{constants, error::AaaSErrorCode, shared, Pool, Vote};

pub fn vote_pool_handler(
    ctx: Context<VotePool>,
    _pool_id: u64,
    candidate: Pubkey,
    is_accepted: bool,
) -> Result<()> {
    let pool = &ctx.accounts.pool;
    require!(
        (pool.end_time + constants::VOTING_TIME) > Clock::get()?.unix_timestamp as u64,
        AaaSErrorCode::LateVoting
    );
    require!(
        pool.end_time < Clock::get()?.unix_timestamp as u64,
        AaaSErrorCode::EarlyVoting
    );
    require!(
        pool.members.binary_search(&candidate.to_string()).is_ok(),
        AaaSErrorCode::OnlyMember
    );

    let voter = ctx.accounts.voter.key;
    let vote = &ctx.accounts.vote;
    require!(
        vote.acceptors.binary_search(voter).is_err(),
        AaaSErrorCode::OneVote
    );
    require!(
        vote.rejectors.binary_search(voter).is_err(),
        AaaSErrorCode::OneVote
    );

    let acceptors_percentage = (vote.acceptors.len() as u16 / pool.members.len() as u16) * 100u16;
    if acceptors_percentage >= constants::WINNING_VOTE_RATE {
        if is_accepted {
            ctx.accounts.vote.acceptors.push(*ctx.accounts.voter.key);
            return Ok(());
        }
        ctx.accounts.vote.rejectors.push(*ctx.accounts.voter.key);
        return Ok(());
    }

    let rejectors_percentage = (vote.rejectors.len() as u16 / pool.members.len() as u16) * 100u16;
    if rejectors_percentage >= constants::LOSING_VOTE_RATE {
        if is_accepted {
            ctx.accounts.vote.acceptors.push(*ctx.accounts.voter.key);
            return Ok(());
        }
        ctx.accounts.vote.rejectors.push(*ctx.accounts.voter.key);
        return Ok(());
    }

    if is_accepted {
        ctx.accounts.vote.acceptors.push(*ctx.accounts.voter.key);
    } else {
        ctx.accounts.vote.rejectors.push(*ctx.accounts.voter.key);
    }
    let acceptors_percentage =
        (ctx.accounts.vote.acceptors.len() as u16 / pool.members.len() as u16) * 100u16;
    if acceptors_percentage >= constants::WINNING_VOTE_RATE {
        shared::transfer_token(
            &ctx.accounts.vault,
            &ctx.accounts.fee_token_account,
            &pool.entry_fee,
            &ctx.accounts.fee_token_mint,
            &ctx.accounts.voter,
            &ctx.accounts.fee_token_program,
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(pool_id: u64, candidate: Pubkey)]
pub struct VotePool<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [b"pool", pool_id.to_le_bytes().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init_if_needed,
        payer=voter,
        space=constants::ANCHOR_DISCRIMINATOR + Vote::INIT_SPACE,
        seeds = [b"vote", pool_id.to_le_bytes().as_ref(), candidate.to_bytes().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,

    #[account(mut)]
    pub fee_token_account: InterfaceAccount<'info, TokenAccount>,

    pub fee_token_mint: InterfaceAccount<'info, Mint>,

    pub fee_token_program: Interface<'info, TokenInterface>,

    #[account(
        mut,
        associated_token::mint = fee_token_mint,
        associated_token::authority = pool,
        associated_token::token_program = fee_token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
