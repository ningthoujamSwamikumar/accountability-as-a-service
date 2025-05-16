use anchor_lang::prelude::*;

#[error_code]
pub enum AaaSErrorCode {
    #[msg("Violated maximum proofs limit!")]
    MaxProofLimit,
    #[msg("Order of time is violated!")]
    TimeOrder,
    #[msg("Max member limit reached!")]
    MaxMember,
    #[msg("Challenge already started!")]
    ChallengeStarted,
    #[msg("Voting time is not reached!")]
    EarlyVoting,
    #[msg("Voting time is over!")]
    LateVoting,
    #[msg("Already voted!")]
    OneVote,
    #[msg("Only members can vote!")]
    OnlyMember,
    #[msg("Already joined!")]
    AlreadyMember,
}
