use anchor_client::{Client, Cluster};
use anchor_lang::AccountDeserialize;
use solana_sdk::signer::Signer;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    system_program::ID as SYSTEM_PROGRAM,
};
use squads_mpl::state::{IncomingInstruction, MsAccountMeta, MsTransaction, MsInstruction};
use std::env;
use std::rc::Rc;
use std::collections::HashMap;
use std::str::FromStr;
use solana_client::nonblocking::rpc_client::RpcClient;
use crate::utils::{MULTISIG_PUBKEY, RPC, SQUADS_PROGRAM_ID};


pub async fn create_transaction(transaction_index: u32) -> Instruction {
    let payer_private_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let payer = Keypair::from_base58_string(&payer_private_key);
    let payer_pubkey = payer.pubkey();

    let cluster = Cluster::from_str(&RPC).unwrap();
    let client = Client::new(cluster, Rc::new(payer));
    let program = client.program(SQUADS_PROGRAM_ID);

    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &MULTISIG_PUBKEY.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    return program
        .request()
        .accounts(squads_mpl::accounts::CreateTransaction {
            system_program: SYSTEM_PROGRAM,
            multisig: MULTISIG_PUBKEY,
            transaction: transaction_pubkey,
            creator: payer_pubkey,
        })
        .args(squads_mpl::instruction::CreateTransaction { authority_index: 1 })
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone();
}

pub async fn add_instruction(transaction_index: u32, instruction_index: u8, instruction: Instruction) -> Instruction {
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &MULTISIG_PUBKEY.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    let (instruction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &transaction_pubkey.to_bytes(),
            &instruction_index.to_le_bytes(),
            b"instruction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    let payer_private_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let payer = Keypair::from_base58_string(&payer_private_key);
    let payer_pubkey = payer.pubkey();

    let cluster = Cluster::from_str(&RPC).unwrap();
    let client = Client::new(cluster, Rc::new(payer));
    let program = client.program(SQUADS_PROGRAM_ID);

    return program
        .request()
        .accounts(squads_mpl::accounts::AddInstruction {
            system_program: SYSTEM_PROGRAM,
            multisig: MULTISIG_PUBKEY,
            transaction: transaction_pubkey,
            creator: payer_pubkey,
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

pub async fn activate_transaction(transaction_index: u32) -> Instruction {
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &MULTISIG_PUBKEY.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );
    let payer_private_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let payer = Keypair::from_base58_string(&payer_private_key);
    let payer_pubkey = payer.pubkey();

    let cluster = Cluster::from_str(&RPC).unwrap();
    let client = Client::new(cluster, Rc::new(payer));
    let program = client.program(SQUADS_PROGRAM_ID);

    return program
        .request()
        .accounts(squads_mpl::accounts::ActivateTransaction {
            multisig: MULTISIG_PUBKEY,
            transaction: transaction_pubkey,
            creator: payer_pubkey,
        })
        .args(squads_mpl::instruction::ActivateTransaction)
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone();
}

pub async fn approve_transaction(transaction_index: u32) -> Instruction {
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &MULTISIG_PUBKEY.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );
    let payer_private_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let payer = Keypair::from_base58_string(&payer_private_key);
    let payer_pubkey = payer.pubkey();

    let cluster = Cluster::from_str(&RPC).unwrap();
    let client = Client::new(cluster, Rc::new(payer));
    let program = client.program(SQUADS_PROGRAM_ID);

    return program
        .request()
        .accounts(squads_mpl::accounts::VoteTransaction {
            multisig: MULTISIG_PUBKEY,
            transaction: transaction_pubkey,
            member: payer_pubkey,
        })
        .args(squads_mpl::instruction::ApproveTransaction)
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone();
}

pub async fn refect_transaction(transaction_index: u32) -> Instruction {
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &MULTISIG_PUBKEY.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );
    let payer_private_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let payer = Keypair::from_base58_string(&payer_private_key);
    let payer_pubkey = payer.pubkey();

    let cluster = Cluster::from_str(&RPC).unwrap();
    let client = Client::new(cluster, Rc::new(payer));
    let program = client.program(SQUADS_PROGRAM_ID);

    return program
        .request()
        .accounts(squads_mpl::accounts::VoteTransaction {
            multisig: MULTISIG_PUBKEY,
            transaction: transaction_pubkey,
            member: payer_pubkey,
        })
        .args(squads_mpl::instruction::RejectTransaction)
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone();
}

pub async fn execute_transaction_ix(transaction_index: u32) -> Instruction {
    let payer_private_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let payer = Keypair::from_base58_string(&payer_private_key);
    let payer_pubkey = payer.pubkey();

    let cluster = Cluster::from_str(&RPC).unwrap();
    let client = Client::new(cluster, Rc::new(payer));
    let program = client.program(SQUADS_PROGRAM_ID);

    let solana_client = RpcClient::new(RPC.to_string());

    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &MULTISIG_PUBKEY.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    let mut transaction_data_bytes = &solana_client
        .get_account_data(&transaction_pubkey)
        .await.unwrap()[..];

    println!(
        "\n\ntransaction_data_bytes: {:?}\n\n",
        transaction_data_bytes
    );

    let ms_transaction_data =
        MsTransaction::try_deserialize(&mut transaction_data_bytes).unwrap();

    println!(
        "\n\ntransaction_data: {}\n\n",
        ms_transaction_data.instruction_index
    );

    let mut instructions_data: Vec<MsInstructionData> = vec![];

    for i in 0..ms_transaction_data.instruction_index {
        let instruction_index = i + 1;

        let (instruction_pubkey, _) = Pubkey::find_program_address(
            &[
                b"squad",
                &transaction_pubkey.to_bytes(),
                &instruction_index.to_le_bytes(),
                b"instruction",
            ],
            &SQUADS_PROGRAM_ID,
        );

        let mut instruction_data_bytes = &solana_client
            .get_account_data(&instruction_pubkey)
            .await
            .unwrap()[..];

        println!(
            "\n\ninstruction_data_bytes: {:?}\n\n",
            instruction_data_bytes
        );

        let instruction_data =
            MsInstruction::try_deserialize(&mut instruction_data_bytes).unwrap();

        println!("\n\ninstruction_data: {}\n\n", instruction_data.program_id);

        instructions_data.push(MsInstructionData {
            pubkey: instruction_pubkey,
            ix_account_data: instruction_data,
        })
    }

    let ix_keys: Vec<InstructionKey> = instructions_data
        .into_iter()
        .flat_map(|ix_account| {
            let formatted_keys: Vec<InstructionKey> = ix_account
                .ix_account_data
                .keys
                .iter()
                .map(|ix_key| InstructionKey {
                    pubkey: ix_key.pubkey,
                    is_signer: false,
                    is_writable: ix_key.is_writable,
                })
                .collect();

            let mut keys = vec![
                InstructionKey {
                    pubkey: ix_account.pubkey,
                    is_signer: false,
                    is_writable: false,
                },
                InstructionKey {
                    pubkey: ix_account.ix_account_data.program_id,
                    is_signer: false,
                    is_writable: false,
                },
            ];

            keys.extend(formatted_keys);
            keys
        })
        .collect();

    let mut unique_map: HashMap<Pubkey, InstructionKey> = HashMap::new();

    for key in &ix_keys {
        // Check if the pubkey already exists in the map
        if let Some(existing_key) = unique_map.get(&key.pubkey) {
            // If it exists, check if the is_writable flag matches
            if existing_key.is_writable == key.is_writable {
                continue; // Skip if it matches
            }
        }
        // Insert or update the key in the map
        unique_map.insert(key.pubkey, key.clone());
    }

    let unique_keys: Vec<InstructionKey> = unique_map.into_iter().map(|(_, v)| v).collect();

    let key_index_array: Vec<isize> = find_key_indices(&ix_keys.clone(), &unique_keys);

    let index_bin = bincode::serialize(&key_index_array).expect("Failed to serialize key index array");

    return program
        .request()
        .accounts(squads_mpl::accounts::ExecuteTransaction {
            multisig: MULTISIG_PUBKEY,
            transaction: transaction_pubkey,
            member: payer_pubkey,
        })
        .args(squads_mpl::instruction::ExecuteTransaction { account_list: index_bin })
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone()
}


pub struct MsInstructionData {
    pubkey: Pubkey,
    ix_account_data: MsInstruction,
}

#[derive(Clone)]
pub struct InstructionKey {
    pubkey: Pubkey,
    is_signer: bool,
    is_writable: bool,
}

pub fn find_key_indices(ix_keys: &[InstructionKey], keys_unique: &[InstructionKey]) -> Vec<isize> {
    ix_keys
        .iter()
        .map(|a| {
            keys_unique
                .iter()
                .position(|k| k.pubkey == a.pubkey && k.is_writable == a.is_writable)
                .map_or(-1, |index| index as isize) // Return -1 if not found, otherwise the index
        })
        .collect()
}
