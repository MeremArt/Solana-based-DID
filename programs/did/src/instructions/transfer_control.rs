// src/instructions/transfer_control.rs
use anchor_lang::prelude::*;
use crate::state::DidAccount;

#[derive(Accounts)]
pub struct TransferControl<'info> {
    #[account(mut)]
    pub did_account: Account<'info, DidAccount>,
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<TransferControl>, new_authority: Pubkey) -> Result<()> {
    let did_account = &mut ctx.accounts.did_account;
    
    // Ensure only the current controller (authority) can transfer control
    require_keys_eq!(
        did_account.authority,
        ctx.accounts.authority.key(),
        crate::error::ErrorCode::Unauthorized
    );
    
    // Update the controller of the DID
    did_account.authority = new_authority;
    
    Ok(())
}