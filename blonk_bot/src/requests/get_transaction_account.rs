use crate::utils::{RPC, SQUADS_PROGRAM_ID};
use anchor_lang::AccountDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use squads_mpl::state::MsTransaction;

pub async fn get_transaction_account(
    multisig_pubkey: Pubkey,
    transaction_index: u32,
) -> MsTransaction {
    let solana_client = RpcClient::new(RPC.to_string());
    let (transaction_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &multisig_pubkey.to_bytes(),
            &transaction_index.to_le_bytes(),
            b"transaction",
        ],
        &SQUADS_PROGRAM_ID,
    );

    let mut transaction_data = &solana_client
        .get_account_data(&transaction_pubkey)
        .await
        .unwrap()[..];

    MsTransaction::try_deserialize(&mut transaction_data).unwrap()
}
