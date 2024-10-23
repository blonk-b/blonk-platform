use crate::utils::SQUADS_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

pub fn get_instruction_pubkey(transaction_pubkey: Pubkey, instruction_index: u8) -> Pubkey {
    let (instruction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &transaction_pubkey.to_bytes(),
            &instruction_index.to_le_bytes(),
            b"instruction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    instruction_pubkey
}
