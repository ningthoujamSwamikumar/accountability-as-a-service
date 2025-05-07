#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

pub mod instructions;
pub use instructions::*;

pub mod states;
pub use states::*;

pub mod constants;
pub use constants::*;

#[program]
pub mod accountability_app {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        pool_id: u64,
        title: String,
        start_time: u64,
        end_time: u64,
        entry_fee: u16,
        accepted_proofs: Vec<String>,
        goal: String,
    ) -> Result<()> {
        if accepted_proofs.len() >= MAX_PROOF_TYPES {
            panic!("exceeded max proof types!");
        }   
        *ctx.accounts.pool = Pool {
            pool_id,
            start_time,
            end_time,
            entry_fee,
            accepted_proofs,
            goal,
            title,
            members: Vec::new()
        };
        Ok(())
    }
}
