use anchor_lang::prelude::*;

use sol_did_cpi::cpi::accounts::{AddService as CpiAddService, Initialize as CpiInitialize};
use sol_did_cpi::cpi::{add_service as cpi_add_service, initialize as cpi_initialize};
use sol_did_cpi::program::SolDid;
use sol_did_cpi::Service;

declare_id!("9kAsUJpZHJFMbYEuAxF1ov4qDJY8pqWoLnDhRd38E4H8");

pub mod state;
pub mod instructions;
pub mod error;

#[program]
pub mod solana_did {
    use super::*;

    pub fn initialize_did(ctx: Context<instructions::InitializeDID>, size: u32) -> Result<()> {
        instructions::initialize_did::handler(ctx, size)
    }

    pub fn add_service(
        ctx: Context<instructions::AddService>,
        fragment: String,
        service_type: String,
        service_endpoint: String,
    ) -> Result<()> {
        instructions::add_service::handler(ctx, fragment, service_type, service_endpoint)
    }

    pub fn transfer_control(ctx: Context<instructions::TransferControl>, new_authority: Pubkey) -> Result<()> {
        instructions::transfer_control::handler(ctx, new_authority)
    }
}