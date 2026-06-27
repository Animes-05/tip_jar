use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer  = owner,
        space  = TIPJAR_SPACE,
        seeds  = [TIP_JAR_SEED, owner.key().as_ref()],
        bump,
    )]
    pub tip_jar: Account<'info, TipJar>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let jar = &mut ctx.accounts.tip_jar;
    jar.owner = *ctx.accounts.owner.key;
    jar.total_tips = 0;
    jar.bump = *ctx.bumps.tip_jar;
    Ok(())
}