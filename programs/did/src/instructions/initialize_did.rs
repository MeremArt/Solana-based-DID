// src/instructions/initialize_did.rs
use anchor_lang::prelude::*;
use sol_did_cpi::cpi::accounts::Initialize as CpiInitialize;
use sol_did_cpi::cpi::initialize as cpi_initialize;
use sol_did_cpi::program::SolDid;

#[derive(Accounts)]
#[instruction(size: u32)]
pub struct InitializeDID<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 // Account discriminator + size of the authority (32 bytes)
    )]
    pub did_account: Account<'info, crate::state::DidAccount>,
    
    #[account(mut)]
    /// CHECK: Checked in the DID program
    pub did_data: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub sol_did_program: Program<'info, SolDid>,
}

pub fn handler(ctx: Context<InitializeDID>, size: u32) -> Result<()> {
    let did_account = &mut ctx.accounts.did_account;
    
    // Set the authority as the signer (controller) of this DID
    did_account.authority = *ctx.accounts.authority.key;
    
    // Call the Solana DID program to initialize the DID
    let cpi_program = ctx.accounts.sol_did_program.to_account_info();
    let cpi_accounts = CpiInitialize {
        did_data: ctx.accounts.did_data.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    cpi_initialize(cpi_ctx, size)
}