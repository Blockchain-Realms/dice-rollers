use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke_signed,
    system_instruction,
};
use chainlink_solana::cpi::request_randomness;
use chainlink_solana::program::ChainlinkSolana;
use chainlink_solana::state::Callback;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
#[instruction(num_sides: u8)]
pub struct RequestRandomness<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = DiceRollState::LEN,
        seeds = [b"dice_roll_state", user.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, DiceRollState>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the Chainlink VRF account
    pub vrf: AccountInfo<'info>,
    /// CHECK: This is the Chainlink program
    pub chainlink_program: Program<'info, ChainlinkSolana>,
    /// System Program
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RequestRandomness>, num_sides: u8) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.num_sides = num_sides;
    state.result = 0;
    state.is_request_pending = true;

    let callback = Callback {
        program_id: *ctx.program_id,
        accounts: vec![
            AccountMeta::new(ctx.accounts.state.key(), false),
            AccountMeta::new_readonly(ctx.accounts.user.key(), true),
        ],
        ix_data: crate::instruction::ConsumeRandomness {}.data(),
    };

    // CPI call to Chainlink VRF to request randomness
    request_randomness(
        CpiContext::new(
            ctx.accounts.chainlink_program.to_account_info(),
            chainlink_solana::cpi::accounts::RequestRandomness {
                requester: ctx.accounts.user.to_account_info(),
                vrf: ctx.accounts.vrf.clone(),
            },
        ),
        callback,
    )?;

    Ok(())
}