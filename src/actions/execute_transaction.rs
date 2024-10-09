use crate::utils::RPC;

use solana_client::client_error::ClientError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{ signature::Signature, signer::{keypair::Keypair, Signer}, message::Message, transaction::Transaction };
use std::env;
use crate::queries::execute_transaction_ix;

pub async fn execute_transaction(transaction_index: u32) -> Result<Signature, ClientError> {
    let solana_client = RpcClient::new(RPC.to_string());
    let instruction = execute_transaction_ix(transaction_index).await;

    let secret_key = env::var("PAYER_PRIVATE_KEY").unwrap();
    let signer = Keypair::from_base58_string(&secret_key);
    let signer_ref = &signer;

    let blockhash = solana_client.get_latest_blockhash().await.unwrap();

    let transaction_message = Message::new(&[instruction], Some(&signer.pubkey()));
    let tx = Transaction::new(&[signer_ref], transaction_message, blockhash);
    let transaction_res = solana_client.send_and_confirm_transaction(&tx).await;

    match transaction_res {
        Ok(res) => {
            println!("EXECUTE SIG: \n\n{}", res);
            Ok(res)
        },
        Err(e) => {
            println!("ERROR EXECUTING TRANSACTION: {}", e);
            Err(e)
        }
    }
}