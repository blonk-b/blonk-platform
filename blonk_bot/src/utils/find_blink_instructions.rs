use anchor_lang::prelude::AccountMeta;
use base64::prelude::*;
use solana_sdk::{instruction::Instruction, transaction::Transaction};

pub fn find_blink_instructions(transaction: String) -> Vec<Instruction> {
    let blink_transaction_as_bytes = BASE64_STANDARD.decode(transaction).unwrap();
    let blink_transaction: Transaction = bincode::deserialize(&blink_transaction_as_bytes).unwrap();

    blink_transaction
        .message
        .instructions
        .iter()
        .map(|instruction| {
            let program_id =
                blink_transaction.message.account_keys[instruction.program_id_index as usize];

            let accounts: Vec<AccountMeta> = instruction
                .accounts
                .iter()
                .map(|account_index| {
                    let pubkey = blink_transaction.message.account_keys[*account_index as usize];

                    match blink_transaction
                        .message
                        .is_writable(*account_index as usize)
                    {
                        true => AccountMeta::new(
                            pubkey,
                            blink_transaction.message.is_signer(*account_index as usize),
                        ),
                        false => AccountMeta::new_readonly(
                            pubkey,
                            blink_transaction.message.is_signer(*account_index as usize),
                        ),
                    }
                })
                .collect();

            Instruction {
                program_id,
                data: instruction.data.clone(),
                accounts,
            }
        })
        .collect()
}
