use anchor_lang::prelude::*;

#[account]
pub struct DiceRollState {
    pub num_sides: u8,
    pub result: u8,
    pub is_request_pending: bool,
}

impl DiceRollState {
    pub const LEN: usize = 8; // Adjust the size as needed
}