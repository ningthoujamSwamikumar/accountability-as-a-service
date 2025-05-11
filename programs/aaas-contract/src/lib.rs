pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("4rKBdcXYihHcYTYb1PTGqGJSYvtC8jPPVPAHfMEHZYmt");

#[program]
pub mod aaas_contract {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        pool_id: u64,
        start_time: u64,
        end_time: u64,
        entry_fee: u16,
        accepted_proofs: Vec<String>,
        goal: String,
    ) -> Result<()> {
        create_pool_handler(
            ctx,
            pool_id,
            start_time,
            end_time,
            entry_fee,
            accepted_proofs,
            goal,
        )
    }

    pub fn join_pool(ctx: Context<JoinPool>, pool_id: u64) -> Result<()> {
        join_pool_handler(ctx, pool_id)
    }
}
