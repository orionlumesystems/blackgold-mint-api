use blackgold_mint_api::prelude::*;
use spl_token::amount_to_ui_amount;
use steel::*;

// # Mint Tokens
//
// Mints BlackGold tokens to the treasury-associated token account.
//
// ## Purpose
//
// This instruction allows the protocol treasury to mint new tokens while
// enforcing all supply and mint frequency restrictions.
//
// Tokens are minted using the program-controlled `MintAuthority` PDA, which
// signs the SPL Token mint operation through PDA seeds.
//
// ## Accounts
//
// | Account | Mut | Signer | Description |
// |----------|------|--------|-------------|
// | Treasury | N | Y | Authorized treasury wallet. |
// | Mint Authority PDA | Y | N | Stores minting state and acts as mint authority. |
// | Token Mint | Y | N | BlackGold SPL mint account. |
// | Treasury ATA | Y | N | Destination account receiving newly minted tokens. |
// | SPL Token Program | N | N | Processes the mint instruction. |
//
// ## Validation
//
// The instruction performs the following checks:
//
// - Treasury account must be the configured treasury address.
// - Treasury account must sign the transaction.
// - Mint Authority account must be a valid program-owned
//   `MintAuthority` account.
// - Mint account must match the configured token mint.
// - Destination account must be the treasury's associated token account.
// - Token Program must be the canonical SPL Token Program.
//
// ## Mint Controls
//
// Before minting, the instruction enforces several protocol-level
// restrictions:
//
// ### Maximum Mint Amount
//
// The requested mint amount must not exceed `MAX_MINT_AMOUNT`.
//
// ### Maximum Supply
//
// The requested mint amount must not cause total token supply to exceed
// `MAX_SUPPLY`.
//
// ### Mint Frequency Limit
//
// The protocol enforces a minimum delay of
// `MIN_SLOTS_BETWEEN_MINT` slots between successful mint operations.
//
// The last successful mint slot is stored in the `MintAuthority` account.
//
// ## State Updates
//
// Upon successful validation:
//
// 1. `last_mint_at` is updated to the current slot.
// 2. The Mint Authority PDA signs the mint operation.
// 3. New tokens are minted into the treasury ATA.
//
// ## PDA Signing
//
// The mint operation is performed through
// `mint_to_signed(...)` using the
// `MINT_AUTHORITY` PDA seeds.
//
// This ensures that only the program-controlled PDA can authorize
// token issuance.
//
// ## Failure Conditions
//
// The instruction fails if:
//
// - Required accounts are missing.
// - Treasury signer validation fails.
// - An invalid mint account is supplied.
// - An invalid destination token account is supplied.
// - The requested amount exceeds `MAX_MINT_AMOUNT`.
// - The mint would exceed `MAX_SUPPLY`.
// - The minimum mint interval has not elapsed.
// - The SPL Token mint instruction fails.
//
// ## Result
//
// New BlackGold tokens are minted to the treasury token account and the
// mint timestamp is updated in the `MintAuthority` PDA.

/// Mints Token to the treasury.
pub fn process_mint_token(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data. (make mint token struct)
    let args = MintToken::try_from_bytes(data)?;

    //* Amount is qty to be minted
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;

    let [signer_info, authority_info, mint_info, to_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //* Signer account (treasury)
    signer_info.is_signer()?.has_address(&TREASURY_ADDRESS)?;

    //* Mint authority
    let mint_authority = authority_info.as_account_mut::<MintAuthority>(&blackgold_mint_api::ID)?;

    //* Mint address
    let mint = mint_info.has_address(&MINT_ADDRESS)?.as_mint()?;

    //* ATA PDA (destination)
    to_info.as_associated_token_account(&TREASURY_ADDRESS, &MINT_ADDRESS)?;

    //* Token program (spl token program)
    token_program.is_program(&spl_token::ID)?;


    // Check max mint amount.
    if amount > MAX_MINT_AMOUNT {
        return Err(trace(
            format!(
                "Cannot mint more than max allowed: {} > {}",
                amount_to_ui_amount(amount, TOKEN_DECIMALS),
                amount_to_ui_amount(MAX_MINT_AMOUNT, TOKEN_DECIMALS)
            )
            .as_str(),
            BlackgoldMintError::MaxAmountExceeded.into(),
        ));
    }

    // Check remaining max supply. By checking current supply and substracting it with max supply
    let remaining_supply = MAX_SUPPLY.saturating_sub(mint.supply());
    if amount > remaining_supply {
        return Err(trace(
            format!(
                "Cannot mint more than remaining supply: {} > {}",
                amount_to_ui_amount(amount, TOKEN_DECIMALS),
                amount_to_ui_amount(remaining_supply, TOKEN_DECIMALS)
            )
            .as_str(),
            BlackgoldMintError::MaxSupplyExceeded.into(),
        ));
    }

    // Check mint frequency.
    let slots_since = clock.slot.saturating_sub(mint_authority.last_mint_at);

    //* Check whether the minimum gap between the last mint time and current request time is OKAY - MIN_SLOTS_BETWEEN_MINT
    if slots_since < MIN_SLOTS_BETWEEN_MINT {
        return Err(trace(
            format!(
                "Must wait at least {} slots between mints: {} slots remaining",
                MIN_SLOTS_BETWEEN_MINT,
                MIN_SLOTS_BETWEEN_MINT.saturating_sub(slots_since)
            )
            .as_str(),
            BlackgoldMintError::MintFrequencyExceeded.into(),
        ));
    }

    //* Update last mint at (timestamp) with current slot
    mint_authority.last_mint_at = clock.slot;

    // Mint tokens to the treasury.
    mint_to_signed(
        mint_info,
        to_info,
        authority_info,
        token_program,
        amount,
        &[MINT_AUTHORITY],
    )?;

    Ok(())
}
