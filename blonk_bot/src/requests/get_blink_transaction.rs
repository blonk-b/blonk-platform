use crate::utils::get_multisig_authority_pubkey;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBlinkTransactionResponse {
    pub transaction: String,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlinkTransactionBody {
    pub account: String,
}

pub async fn get_blink_transaction(
    multisig_pubkey: Pubkey,
    url: &String,
) -> Result<GetBlinkTransactionResponse, Error> {
    let client = Client::new();
    let multisig_authority_pubkey = get_multisig_authority_pubkey(multisig_pubkey, 1);
    let body = BlinkTransactionBody {
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
            let transaction = res.json::<GetBlinkTransactionResponse>().await;

            match transaction {
                Ok(response) => {
                    println!("ACTION TRANASCTION: \n\n{:?}", response);
                    Ok(response)
                }
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
