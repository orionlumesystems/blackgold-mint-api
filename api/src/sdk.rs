use steel::*;

use crate::{
    consts::{MINT_ADDRESS, TREASURY_ADDRESS},
    instruction::*,
    state::*,
};

pub fn init(signer: Pubkey) -> Instruction {
    let mint_authority_address = mint_authority_pda().0;

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint_authority_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Init {}.to_bytes(),
    }
}

pub fn mint_blackgold(amount: u64) -> Instruction {
    let signer_address = TREASURY_ADDRESS;
    let mint_authority_address = mint_authority_pda().0;
    let to_address =
        spl_associated_token_account::get_associated_token_address(&signer_address, &MINT_ADDRESS);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer_address, true),
            AccountMeta::new(mint_authority_address, false),
            AccountMeta::new(MINT_ADDRESS, false),
            AccountMeta::new(to_address, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: MintToken {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
