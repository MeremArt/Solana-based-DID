// src/instructions/add_service.rs
use anchor_lang::prelude::*;
use sol_did_cpi::cpi::accounts::AddService as CpiAddService;
use sol_did_cpi::cpi::add_service as cpi_add_service;
use sol_did_cpi::program::SolDid;
use sol_did_cpi::Service;

#[derive(Accounts)]
pub struct AddService<'info> {
    #[account(mut)]
    pub did_account: Account<'info, crate::state::DidAccount>,
    
    #[account(mut)]
    pub did_data: UncheckedAccount<'info>,
    
    pub authority: Signer<'info>,
    
    pub sol_did_program: Program<'info, SolDid>,
}

pub fn handler(
    ctx: Context<AddService>,
    fragment: String,
    service_type: String,
    service_endpoint: String,
) -> Result<()> {
    let cpi_program = ctx.accounts.sol_did_program.to_account_info();
    let cpi_accounts = CpiAddService {
        did_data: ctx.accounts.did_data.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    cpi_add_service(
        cpi_ctx,
        Service {
            fragment,
            service_type,
            service_endpoint,
        },
        false,
        None,
    )
}