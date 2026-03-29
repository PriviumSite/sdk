use anchor_lang::prelude::*;
use crate::state::*;

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.authority = *ctx.accounts.authority.key;
    Ok(())
}

pub fn shield_funds(ctx: Context<ShieldFunds>, amount: u64) -> Result<()> {
    msg!("Shielding {} tokens", amount);
    Ok(())
}
