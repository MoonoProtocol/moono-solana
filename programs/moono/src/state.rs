use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
    pub version: u8,
    pub bump: u8,
    pub authority: Pubkey,
    pub paused: bool,
}

#[account]
#[derive(InitSpace)]
pub struct AssetPool {
    pub version: u8,
    pub bump: u8,
    pub protocol: Pubkey,
    pub mint: Pubkey,
    pub is_enabled: bool,
    pub allow_deposits: bool,
    pub allow_borrows: bool,
    pub decimals: u8,
}