use blackgold_mint_api::prelude::*;
use steel::*;

/// Initializes the Mint Authority PDA.
///
/// Creates the singleton `MintAuthority` account derived from
/// `[MINT_AUTHORITY]` and owned by the BlackGold Mint program.
///
/// # Accounts
///
/// 1. `[signer]` Treasury/Payer account.
/// 2. `[writable]` Mint Authority PDA.
/// 3. `[]` System Program.
///
/// # Validation
///
/// - Treasury account must be a signer.
/// - Mint Authority PDA must not already exist.
/// - PDA must match the expected seed derivation.
/// - System Program must be the canonical system program.
///
/// # Initialization
///
/// Creates and allocates the PDA account, writes the discriminator,
/// and initializes `last_mint_at` to the current slot.
///
/// 1. Invokes system_program::create_account to allocate the PDA.
/// 2. Assigns the PDA to this program (blackgold_mint_api::ID).
/// 3. Allocates enough space for the MintAuthority struct.
/// 4. Writes the account discriminator (BlackgoldAccount::MintAuthority = 100).
/// 5. Initializes the account data buffer to zero.
/// 6. Returns a fully created, writable PDA ready for mutation.
///
/// # Errors
///
/// Returns an error if:
/// - Required accounts are missing.
/// - PDA validation fails.
/// - The PDA already exists.
/// - Account creation fails.
/// - The System Program account is invalid.
/// Initializes the mint authority account.
pub fn process_init(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    //* Load accounts.
    let clock = Clock::get()?;

    //* Destructure the accounts array into the expected 3 accounts
    let [signer_info, authority_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //* Treasury Account (Signer)
    signer_info.is_signer()?;

    //* Mint Authority PDA Account
    authority_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[MINT_AUTHORITY], &blackgold_mint_api::ID)?;

    //* System Program
    system_program.is_program(&system_program::ID)?;

    //* Create Program Account
    create_program_account::<MintAuthority>(
        authority_info,
        system_program,
        signer_info,
        &blackgold_mint_api::ID,
        &[MINT_AUTHORITY],
    )?;

    //* Interpret this account's data buffer as a MintAuthority struct, but only if it is owned by my program.
    let mint_authority = authority_info.as_account_mut::<MintAuthority>(&blackgold_mint_api::ID)?;

    //* Set current time to last_mint_at
    mint_authority.last_mint_at = clock.slot;

    //* Saves the change to the struct
    Ok(())
}
