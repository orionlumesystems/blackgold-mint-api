use solana_program::pubkey::{pubkey, Pubkey};

/// The decimal precision of the token.
/// There are 100 billion indivisible units per unit of token (called "grams").
pub const TOKEN_DECIMALS: u8 = 11;

/// One token, denominated in indivisible units.
pub const ONE_OUR_TOKEN: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The maximum amount of tokens that can be minted per request (one request).
pub const MAX_MINT_AMOUNT: u64 = ONE_OUR_TOKEN * 2;

/// The number of slots required between mint requests.
pub const MIN_SLOTS_BETWEEN_MINT: u64 = 150;

/// The maximum token supply (21 million).
pub const MAX_SUPPLY: u64 = ONE_OUR_TOKEN * 21_000_000;

/// The seed of the automation account PDA. (Seed of mint authority)
pub const MINT_AUTHORITY: &[u8] = b"authority";

/// The address of the mint account. (Account of token)
pub const MINT_ADDRESS: Pubkey = pubkey!("BLKGmSnWUj7EYirCirqgnS1Wvi3JwbCbHk2Ws2baLuNt");

/// The treasury address allowed to request a mint. (Account of treasury)
pub const TREASURY_ADDRESS: Pubkey = pubkey!("treo2Q28ihT6e6T73dMoQGwYrsFDr54aNruVVMWzqXz");
