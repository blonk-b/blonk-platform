use solana_sdk::signature::Keypair;
use std::env;
use teloxide::types::UserId;

pub fn get_user_keypair(user_id: UserId) -> Keypair {
    let whitelist = env::var("WHITELIST").unwrap();

    let parsed_whitelist: Vec<(String, String)> = serde_json::from_str(&whitelist).unwrap();

    let private_key = &parsed_whitelist
        .iter()
        .find(|entry| entry.0 == user_id.to_string())
        .unwrap()
        .1;

    Keypair::from_base58_string(private_key)
}
