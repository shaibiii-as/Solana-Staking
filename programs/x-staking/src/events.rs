//! Crate events

use anchor_lang::prelude::*;


#[event]
#
pub struct TreasuryCreated {
    pub label: String,
    pub authority: Pubkey,
    pub treasury_mint: Pubkey,
    pub treasury_vault: Pubkey,
    pub pos_mint: Pubkey 
}

#[event]
pub struct Deposited {
    pub label: String,
    pub _amount: u64,
    pub by: Pubkey,
}

#[event]
pub struct Claimed {
    pub label:String,
    pub _amount: u64,
    pub by: Pubkey
}