use anchor_lang::prelude::*;
use chainlink_solana::state::VrfAccountData;
use crate::state::*;
use crate::error::*;

#[derive(Accounts)]
pub struct ConsumeRandomness<'info> {
    #[account(mut, seeds = [b"dice_roll_state", user.key().as_ref()], bump)]
    pub state: Account<'info, DiceRollState>,
    /// CHECK: This is the Chainlink VRF account
    pub vrf: AccountInfo<'info>,
    /// The user who requested the randomness
    pub user: Signer<'info>,
}

pub fn handler(ctx: Context<ConsumeRandomness>) -> Result<()> {
    let state = &mut ctx.accounts.state;

    // Ensure there is a pending request
    if !state.is_request_pending {
        return Err(DiceRollError::NoPendingRequest.into());
    }

    // Deserialize the VRF account data to get the randomness
    let vrf_data = VrfAccountData::try_from_slice(&ctx.accounts.vrf.data.borrow())?;

    // Get the randomness value
    let randomness = vrf_data.randomness.ok_or(DiceRollError::RandomnessNotAvailable)?;

    // Calculate the dice roll result
    let result = (randomness[0] % state.num_sides) + 1;
    state.result = result;
    state.is_request_pending = false;

    msg!("Dice roll result: {}", result);

    Ok(())
}