use anchor_lang::prelude::*;

#[account]
pub struct TipJar {
    pub owner: Pubkey,
    pub total_tips: u64,
    pub bump: u8,
}
