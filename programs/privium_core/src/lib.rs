use anchor_lang::prelude::*;

pub mod instruction;
pub mod processor;
pub mod state;

use processor::*;

declare_id!("Privium1111111111111111111111111111111111");

#[program]
pub mod privium_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        processor::initialize(ctx)
    }

    pub fn shield_funds(ctx: Context<ShieldFunds>, amount: u64) -> Result<()> {
        processor::shield_funds(ctx, amount)
    }
}
