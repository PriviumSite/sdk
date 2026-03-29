use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32)]
    pub state: Account<'info, PriviumState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ShieldFunds<'info> {
    #[account(mut)]
    pub state: Account<'info, PriviumState>,
}
