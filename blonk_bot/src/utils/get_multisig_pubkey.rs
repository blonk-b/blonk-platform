use solana_sdk::pubkey::Pubkey;
use std::{env, str::FromStr};

pub fn get_multisig_pubkey() -> Pubkey {
    let multisig_pubkey = env::var("MULTISIG_PUBKEY").unwrap();

    Pubkey::from_str(&multisig_pubkey).unwrap()
}
