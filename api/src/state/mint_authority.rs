use serde::{Deserialize, Serialize};
use steel::*;

use super::BlackgoldAccount;

/// Account which has the mint authority for the token.
/// Mint authority struct to store last mint at data.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct MintAuthority {
    /// The slot of the last mint.
    pub last_mint_at: u64,
}

account!(BlackgoldAccount, MintAuthority);
