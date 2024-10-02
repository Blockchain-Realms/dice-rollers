use anchor_lang::prelude::*;

#[error_code]
pub enum DiceRollError {
    #[msg("No pending randomness request.")]
    NoPendingRequest,
    #[msg("Randomness not yet available.")]
    RandomnessNotAvailable,
}