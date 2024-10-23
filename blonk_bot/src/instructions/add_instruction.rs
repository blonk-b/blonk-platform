use crate::utils::{
    get_instruction_pubkey, get_program, get_transaction_pubkey, get_user_keypair,
    SQUADS_PROGRAM_ID,
};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program};
use squads_mpl::state::{IncomingInstruction, MsAccountMeta};
use teloxide::types::UserId;

pub fn add_instruction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    instruction_index: u8,
    instruction: Instruction,
    user_id: UserId,
) -> Instruction {
    let creator_keypair = get_user_keypair(user_id);
    let creator_pubkey = creator_keypair.pubkey();
    let program = get_program(creator_keypair, SQUADS_PROGRAM_ID);
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);
    let instruction_pubkey = get_instruction_pubkey(transaction_pubkey, instruction_index);

    return program
        .request()
        .accounts(squads_mpl::accounts::AddInstruction {
            system_program: system_program::ID,
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            creator: creator_pubkey,
            instruction: instruction_pubkey,
        })
        .args(squads_mpl::instruction::AddInstruction {
            incoming_instruction: IncomingInstruction {
                data: instruction.data,
                program_id: instruction.program_id,
                keys: instruction
                    .accounts
                    .iter()
                    .map(|account| MsAccountMeta {
                        is_signer: account.is_signer,
                        is_writable: account.is_writable,
                        pubkey: account.pubkey,
                    })
                    .collect(),
            },
        })
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone();
}
