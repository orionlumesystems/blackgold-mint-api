// -------------------------------
// mod.rs module
// -------------------------------

mod mint_authority;

pub use mint_authority::*;

use crate::consts::*;
use steel::*;

// ---------------------------------------------
// BlackgoldAccount enum
// ---------------------------------------------
// Each on‑chain account type gets a unique discriminator.
//
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum BlackgoldAccount {
    MintAuthority = 100,
}

// ---------------------------------------------
// mint_authority_pda()
// ---------------------------------------------
// Derives the PDA used as the mint authority.
//
pub fn mint_authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINT_AUTHORITY], &crate::ID)
}
