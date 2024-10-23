use crate::utils::get_instruction_pubkey;
use anchor_client::Program;
use anchor_lang::prelude::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use squads_mpl::state::{MsInstruction, MsTransaction};

pub fn get_transaction_account_metas(
    program: &Program,
    transaction_pubkey: Pubkey,
) -> Vec<AccountMeta> {
    let transaction: MsTransaction = program.account(transaction_pubkey).unwrap();

    let mut instructions: Vec<(Pubkey, MsInstruction)> = vec![];

    for i in 0..transaction.instruction_index {
        let instruction_index = i + 1;
        let instruction_pubkey = get_instruction_pubkey(transaction_pubkey, instruction_index);
        let instruction: MsInstruction = program.account(instruction_pubkey).unwrap();

        instructions.push((instruction_pubkey, instruction))
    }

    instructions
        .into_iter()
        .flat_map(|(pubkey, instruction)| {
            let formatted_keys: Vec<AccountMeta> = instruction
                .keys
                .iter()
                .map(|instruction_key| AccountMeta {
                    pubkey: instruction_key.pubkey,
                    is_signer: false,
                    is_writable: instruction_key.is_writable,
                })
                .collect();

            let mut keys = vec![
                AccountMeta {
                    pubkey,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: instruction.program_id,
                    is_signer: false,
                    is_writable: false,
                },
            ];

            keys.extend(formatted_keys);
            keys
        })
        .collect()
}
