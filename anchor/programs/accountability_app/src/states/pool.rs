use anchor_lang::prelude::*;

use crate::MAX_PROOF_TYPES;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub pool_id: u64,
    #[max_len(50)]
    pub title: String,
    pub start_time: u64,
    pub end_time: u64,
    pub entry_fee: u16,
    #[max_len(MAX_PROOF_TYPES, 50)]
    pub accepted_proofs: Vec<String>,
    #[max_len(50)]
    pub goal: String,
    #[max_len(20, 256)]
    pub members: Vec<String>,
}
