mod init;
mod mint_token;

use init::*;
use mint_token::*;

use blackgold_mint_api::instruction::*;
use solana_security_txt::security_txt;
use steel::*;

// # BlackGold Mint Program
//
// Program entrypoint and instruction dispatcher for the BlackGold minting
// protocol.
//
// ## Overview
//
// This program exposes a constrained token minting mechanism controlled by
// a program-derived Mint Authority.
//
// Supported instructions:
//
// - `Init`
//   - Creates and initializes the `MintAuthority` PDA.
//
// - `MintToken`
//   - Mints new tokens to the treasury token account.
//   - Enforces protocol supply and mint frequency restrictions.
//
// ## Instruction Routing
//
// Incoming instruction data is validated and decoded using
// `parse_instruction(...)`.
//
// The decoded instruction is then dispatched to the corresponding processor:
//
// - `process_init()`
// - `process_mint_token()`
//
// ## Security
//

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&blackgold_mint_api::ID, program_id, data)?;

    match ix {
        MintInstruction::Init => process_init(accounts, data)?,
        MintInstruction::MintToken => process_mint_token(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

//TODO
security_txt! {
    name: "BLKG Mint",
    project_url: "https://blackgold.supply",
    contacts: "email:druscilla2024@gmail.com",
    policy: "",
    preferred_languages: "en",
    source_code: "https://github.com/orionlumesystems/blackgold-mint-api"
}
