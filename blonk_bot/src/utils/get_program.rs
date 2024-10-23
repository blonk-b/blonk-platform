use anchor_client::{Client, Cluster, Program};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::{rc::Rc, str::FromStr};

pub fn get_program(signer: Keypair, program_id: Pubkey) -> Program {
    let cluster = Cluster::from_str(crate::utils::RPC).unwrap();
    let client = Client::new(cluster, Rc::new(signer));

    client.program(program_id)
}
