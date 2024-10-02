use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod utils;

use state::*;
use error::*;

declare_id!("YourProgramIDGoesHere...");

#[program]
pub mod dice_roller {
    use super::*;

    pub fn request_randomness(ctx: Context<RequestRandomness>, num_sides: u8) -> Result<()> {
        instructions::request_randomness::handler(ctx, num_sides)
    }

    pub fn consume_randomness(ctx: Context<ConsumeRandomness>) -> Result<()> {
        instructions::consume_randomness::handler(ctx)
    }
}