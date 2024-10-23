use crate::utils::RPC;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signature},
    transaction::Transaction,
};

pub async fn send_and_confirm_transaction(message: Message, signers: Vec<&Keypair>) -> Signature {
    let solana_client = RpcClient::new(RPC.to_string());
    let blockhash = solana_client.get_latest_blockhash().await.unwrap();
    let transaction = Transaction::new(&signers, message, blockhash);

    solana_client
        .send_and_confirm_transaction(&transaction)
        .await
        .unwrap()
}
