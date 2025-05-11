use anchor_lang::prelude::*;

#[error_code]
pub enum AaaSErrorCode {
    #[msg("Violated maximum proofs limit!")]
    MaxProofLimit,
    #[msg("Order of time is violated!")]
    TimeOrder,
    #[msg("Max member limit reached!")]
    MaxMember,
}
