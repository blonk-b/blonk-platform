use crate::{requests::send_and_confirm_transaction, utils::get_user_keypair};
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};
use teloxide::types::UserId;

pub async fn reject_transaction(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
    user_id: UserId,
) -> Signature {
    let instructions: Vec<Instruction> = vec![crate::instructions::reject_transaction(
        multisig_pubkey,
        transaction_index,
        user_id,
    )];

    let member_keypair = get_user_keypair(user_id);
    let member_pubkey = member_keypair.pubkey();
    let message = Message::new(&instructions, Some(&member_pubkey));
    let signers: Vec<&Keypair> = vec![&member_keypair];

    send_and_confirm_transaction(message, signers).await
}
