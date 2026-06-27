use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::constants::*;
use crate::error::*;
use crate::state::*;

pub fn handler(ctx: Context<Tip>, amount: u64) -> Result<()> {
    require!(amount > 0, TipJarError::ZeroAmount);

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.tipper.to_account_info(),
                to: ctx.accounts.tip_jar.to_account_info(),
            },
        ),
        amount,
    )?;

    let jar = &mut ctx.accounts.tip_jar;
    jar.total_tips = jar
    
        .total_tips
        .checked_add(amount)
        .ok_or(TipJarError::Overflow)?;
    Ok(())
}

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(
        mut,
        seeds = [TIP_JAR_SEED, tip_jar.owner.as_ref()],
        bump = tip_jar.bump,
    )]
    pub tip_jar: Account<'info, TipJar>,

    #[account(mut)]
    pub tipper: Signer<'info>,
    pub system_program: Program<'info, System>,
}