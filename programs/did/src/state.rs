
// state.rs
use anchor_lang::prelude::*;

#[account]
pub struct DidAccount {
    pub authority: Pubkey,
}
