use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

pub mod state;
use state::*;

declare_id!("moonoL26kRC8S49yPuuopKhbNhvgf2h4Dva91noD8rN");

#[program]
pub mod moono {
    use super::*;

    pub fn ping(_ctx: Context<Ping>) -> Result<()> {
        msg!("hello world");
        Ok(())
    }

    pub fn initialize_protocol(ctx: Context<InitializeProtocol>) -> Result<()> {
        let protocol = &mut ctx.accounts.protocol;

        protocol.version = 1;
        protocol.authority = ctx.accounts.authority.key();
        protocol.paused = false;
        protocol.bump = ctx.bumps.protocol;

        msg!("Protocol initialized");

        Ok(())
    }

    pub fn initialize_asset_pool(ctx: Context<InitializeAssetPool>) -> Result<()> {
        let asset_pool = &mut ctx.accounts.asset_pool;
        let protocol = &ctx.accounts.protocol;

        asset_pool.version = 1;
        asset_pool.bump = ctx.bumps.asset_pool;
        asset_pool.protocol = protocol.key();
        asset_pool.mint = ctx.accounts.mint.key();
        asset_pool.is_enabled = true;
        asset_pool.allow_deposits = true;
        asset_pool.allow_borrows = true;
        asset_pool.decimals = ctx.accounts.mint.decimals;

        msg!("Asset pool initialized");

        Ok(())
    }

    pub fn set_asset_pool_flags(
        ctx: Context<SetAssetPoolFlags>,
        is_enabled: bool,
        allow_deposits: bool,
        allow_borrows: bool,
    ) -> Result<()> {
        let protocol = &ctx.accounts.protocol;
        let asset_pool = &mut ctx.accounts.asset_pool;

        require!(
            protocol.authority == ctx.accounts.authority.key(),
            MoonoError::Unauthorized
        );

        asset_pool.is_enabled = is_enabled;
        asset_pool.allow_deposits = allow_deposits;
        asset_pool.allow_borrows = allow_borrows;

        msg!("Asset pool flags updated");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ping {}

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"protocol"],
        bump,
        space = 8 + ProtocolConfig::INIT_SPACE
    )]
    pub protocol: Account<'info, ProtocolConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAssetPool<'info> {
    #[account(
        mut,
        seeds = [b"protocol"],
        bump = protocol.bump,
        has_one = authority
    )]
    pub protocol: Account<'info, ProtocolConfig>,

    #[account(
        init,
        payer = authority,
        seeds = [b"asset_pool", mint.key().as_ref()],
        bump,
        space = 8 + AssetPool::INIT_SPACE
    )]
    pub asset_pool: Account<'info, AssetPool>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAssetPoolFlags<'info> {
    #[account(
        seeds = [b"protocol"],
        bump = protocol.bump,
        has_one = authority
    )]
    pub protocol: Account<'info, ProtocolConfig>,

    #[account(
        mut,
        seeds = [b"asset_pool", asset_pool.mint.as_ref()],
        bump = asset_pool.bump,
        constraint = asset_pool.protocol == protocol.key()
    )]
    pub asset_pool: Account<'info, AssetPool>,

    pub authority: Signer<'info>,
}

#[error_code]
pub enum MoonoError {
    #[msg("Unauthorized")]
    Unauthorized,
}
