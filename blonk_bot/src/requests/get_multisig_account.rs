use crate::utils::RPC;
use anchor_lang::AccountDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use squads_mpl::state::Ms;

pub async fn get_multisig_account(multisig_pubkey: Pubkey) -> Ms {
    let solana_client = RpcClient::new(RPC.to_string());
    let mut multisig_data = &solana_client
        .get_account_data(&multisig_pubkey)
        .await
        .unwrap()[..];

    Ms::try_deserialize(&mut multisig_data).unwrap()
}
