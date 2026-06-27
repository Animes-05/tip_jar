use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::*;
use crate::state::*;

pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
    let jar = &ctx.accounts.tip_jar;
    let jar_info = jar.to_account_info();

    let rent = Rent::get()?;
    let min_balance = rent.minimum_balance(TIPJAR_SPACE);

    let withdraw_amount = jar_info
        .lamports()
        .checked_sub(min_balance)
        .ok_or(TipJarError::NothingToWithdraw)?;

    require!(withdraw_amount > 0, TipJarError::NothingToWithdraw);

    **jar_info.try_borrow_mut_lamports()? = withdraw_amount;
    **ctx
        .accounts
        .owner
        .to_account_info()
        .try_borrow_mut_lamports()? += withdraw_amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [TIP_JAR_SEED, tip_jar.owner.as_ref()],
        bump = tip_jar.bump,
        has_one = owner,
    )]
    pub tip_jar: Account<'info, TipJar>,

    #[account(mut)]
    pub owner: Signer<'info>,
}