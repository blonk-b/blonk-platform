use crate::utils::{ MULTISIG_PUBKEY, RPC };
use anchor_lang::{prelude::AccountMeta, AccountDeserialize};
use base64::prelude::*;
use solana_client::client_error::ClientError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, message::Message, signer::{keypair::Keypair, Signer}, transaction::Transaction
};
use squads_mpl::state::Ms;
use std::env;

pub async fn create_transaction(url: &String) -> Result<u32, ClientError> {
    let transaction_response = crate::queries::generate_transaction(&url).await.unwrap();

    let solana_client = RpcClient::new(RPC.to_string());

    let mut multisig_data = &solana_client
        .get_account_data(&MULTISIG_PUBKEY)
        .await
        .unwrap()[..];
    let multisig = Ms::try_deserialize(&mut multisig_data).unwrap();
    let transaction_index = multisig.transaction_index + 1;

    let mut instructions: Vec<Instruction> = vec![];

    instructions.push(crate::queries::create_transaction(transaction_index).await);

    let transaction_response_as_bytes = BASE64_STANDARD
        .decode(transaction_response.transaction)
        .unwrap();
    let blink_transaction: Transaction =
        bincode::deserialize(&transaction_response_as_bytes).unwrap();

    let blink_instructions: Vec<Instruction> = blink_transaction
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
        .collect();

    let mut instruction_index: u8 = 1;

    for instruction in blink_instructions {
        instructions.push(
            crate::queries::add_instruction(transaction_index, instruction_index, instruction)
                .await,
        );
        instruction_index += 1;
    }

    instructions.push(crate::queries::activate_transaction(transaction_index).await);

    instructions.push(crate::queries::approve_transaction(transaction_index).await);

    let secret_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let signer = Keypair::from_base58_string(&secret_key);
    let signer_ref = &signer;

    let blockhash = solana_client.get_latest_blockhash().await.unwrap();

    let message = Message::new(&instructions, Some(&signer.pubkey()));

    let tx = Transaction::new(&[signer_ref], message, blockhash);
    let transaction_res = solana_client.send_and_confirm_transaction(&tx).await;

    match transaction_res {
        Ok(res) => {
            println!("SIGNATURE: \n\n{}", res);

            Ok(transaction_index)
        }
        Err(e) => {
            println!("RES ERROR: {}", e);
            Err(e)
        }
    }
}

