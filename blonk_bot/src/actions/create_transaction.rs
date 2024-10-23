use crate::{
    collections::Transaction,
    requests::{get_blink_transaction, get_multisig_account, send_and_confirm_transaction},
    utils::{find_blink_instructions, get_user_keypair},
};
use solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer,
};
use teloxide::types::UserId;

pub async fn create_transaction(
    url: &String,
    multisig_pubkey: Pubkey,
    user_id: UserId,
) -> Transaction {
    let multisig_account = get_multisig_account(multisig_pubkey).await;
    let transaction_index = multisig_account.transaction_index + 1;

    let mut instructions: Vec<Instruction> = vec![];

    instructions.push(crate::instructions::create_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    ));

    let mut instruction_index: u8 = 1;
    let get_blink_transaction_response = get_blink_transaction(multisig_pubkey, url).await.unwrap();
    let blink_instructions = find_blink_instructions(get_blink_transaction_response.transaction);

    for instruction in blink_instructions {
        instructions.push(crate::instructions::add_instruction(
            multisig_pubkey,
            transaction_index,
            instruction_index,
            instruction,
            user_id,
        ));
        instruction_index += 1;
    }

    instructions.push(crate::instructions::activate_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    ));

    instructions.push(crate::instructions::approve_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    ));

    let creator_keypair = get_user_keypair(user_id);
    let creator_pubkey = creator_keypair.pubkey();
    let message = Message::new(&instructions, Some(&creator_pubkey));
    let signers: Vec<&Keypair> = vec![&creator_keypair];
    let signature = send_and_confirm_transaction(message, signers).await;

    crate::requests::create_transaction(
        transaction_index.try_into().unwrap(),
        user_id,
        signature.to_string(),
    )
    .await
}
