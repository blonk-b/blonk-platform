use crate::requests::get_transaction_account_metas;
use crate::utils::{
    find_key_indexes, find_unique_account_metas_map, get_program, get_transaction_pubkey,
    get_user_keypair, SQUADS_PROGRAM_ID,
};
use anchor_lang::prelude::AccountMeta;
use solana_sdk::signer::Signer;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use teloxide::types::UserId;

pub fn execute_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Instruction {
    let member_keypair = get_user_keypair(user_id);
    let member_pubkey = member_keypair.pubkey();
    let program = get_program(member_keypair, SQUADS_PROGRAM_ID);
    let transaction_pubkey = get_transaction_pubkey(multisig_pubkey, transaction_index);
    let account_metas = get_transaction_account_metas(&program, transaction_pubkey);
    let unique_account_metas_map = find_unique_account_metas_map(&account_metas);
    let unique_account_metas: Vec<AccountMeta> = unique_account_metas_map.into_values().collect();
    let key_index_array: Vec<u8> = find_key_indexes(&account_metas.clone(), &unique_account_metas);

    let mut execute_ix = program
        .request()
        .accounts(squads_mpl::accounts::ExecuteTransaction {
            multisig: multisig_pubkey,
            transaction: transaction_pubkey,
            member: member_pubkey,
        })
        .args(squads_mpl::instruction::ExecuteTransaction {
            account_list: key_index_array,
        })
        .instructions()
        .unwrap()
        .first()
        .unwrap()
        .clone();

    execute_ix.accounts.extend(unique_account_metas);

    execute_ix
}
