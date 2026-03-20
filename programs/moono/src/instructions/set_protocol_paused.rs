use anchor_lang::prelude::*;

use crate::errors::MoonoError;
use crate::state::ProtocolConfig;

pub fn handle_set_protocol_paused(
    ctx: Context<SetProtocolPaused>,
    paused: bool,
) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;

    require!(protocol.authority == ctx.accounts.authority.key(), MoonoError::Unauthorized);

    protocol.paused = paused;

    msg!("Protocol paused flag updated");
    Ok(())
}

#[derive(Accounts)]
pub struct SetProtocolPaused<'info> {
    #[account(
        mut,
        seeds = [b"protocol"],
        bump = protocol.bump,
        has_one = authority
    )]
    pub protocol: Account<'info, ProtocolConfig>,

    pub authority: Signer<'info>,
}
