use anchor_lang::prelude::*;

#[account]
pub struct PriviumState {
    pub authority: Pubkey,
}
