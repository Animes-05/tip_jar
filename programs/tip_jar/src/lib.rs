pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7n7uxngyUJtQVFvhCVatkHcMV2GfbvN2D1nk7EFcBKMV");

#[program]
pub mod tip_jar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

CPI.
    pub fn tip(ctx: Context<Tip>, amount: u64) -> Result<()> {
        instructions::tip::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw::handler(ctx)
    }
}
