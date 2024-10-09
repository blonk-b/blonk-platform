use crate::{
    collections::{BlinkTransaction, ReqBody},
    utils::{MULTISIG_PUBKEY, SQUADS_PROGRAM_ID},
};
use reqwest::{Client, Error};
use solana_sdk::pubkey::Pubkey;

pub async fn generate_transaction(url: &String) -> Result<BlinkTransaction, Error> {
    let client = Client::new();

    let authority: u32 = 1;
    let (multisig_authority_pubkey, _) = Pubkey::find_program_address(
        &[
            b"squad",
            &MULTISIG_PUBKEY.to_bytes(),
            &authority.to_le_bytes(),
            b"authority",
        ],
        &SQUADS_PROGRAM_ID,
    );
    let body = ReqBody {
        account: multisig_authority_pubkey.to_string(),
    };

    let blink_response = client
        .post(url)
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await;

    match blink_response {
        Ok(res) => {
            let transaction = res.json::<BlinkTransaction>().await;

            match transaction {
                Ok(response) => {
                    println!("ACTION TRANASCTION: \n\n{:?}", response);
                    Ok(response)
                },
                Err(e) => {
                    println!("Transaction failed: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("POST request failed: {}", e);
            Err(e)
        }
    }
}
