use crate::utils::SQUADS_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

pub fn get_multisig_authority_pubkey(multisig_pubkey: Pubkey, authority: u32) -> Pubkey {
    let (multisig_authority_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &multisig_pubkey.to_bytes(),
            &authority.to_le_bytes(),
            b"authority",
        ],
        &SQUADS_PROGRAM_ID,
    );

    multisig_authority_pubkey
}
