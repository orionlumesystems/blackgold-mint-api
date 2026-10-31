use serde::{Deserialize, Serialize};
use steel::*;

use super::BlackgoldAccount;

/// Account which has the mint authority for the token.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct MintAuthority {
    //* Mint authority struct to store last mint at data
    /// The slot of the last mint.
    pub last_mint_at: u64,
}

account!(BlackgoldAccount, MintAuthority);
