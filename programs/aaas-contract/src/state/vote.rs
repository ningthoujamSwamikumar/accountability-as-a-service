use anchor_lang::prelude::*;

use crate::constants::MAX_MEMBER;

#[account]
#[derive(InitSpace)]
pub struct Vote {
    pub candidate: Pubkey,
    #[max_len(MAX_MEMBER, 256)]
    pub acceptors: Vec<Pubkey>,
    #[max_len(MAX_MEMBER, 256)]
    pub rejectors: Vec<Pubkey>,
}
