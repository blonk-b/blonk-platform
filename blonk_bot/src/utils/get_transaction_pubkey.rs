use crate::utils::SQUADS_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

pub fn get_transaction_pubkey(multisig_pubkey: Pubkey, transaction_index: u32) -> Pubkey {
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &multisig_pubkey.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    transaction_pubkey
}
