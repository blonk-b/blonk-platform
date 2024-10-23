use crate::collections::Transaction;
use serde::{Deserialize, Serialize};
use std::env;
use teloxide::types::UserId;

#[derive(Serialize, Deserialize)]
pub struct CreateTransactionBody {
    pub transaction_index: i32,
    pub user_id: String,
    pub signature: String,
}

pub async fn create_transaction(
    transaction_index: i32,
    user_id: UserId,
    signature: String,
) -> Transaction {
    let base_url = env::var("API_BASE_URL").unwrap();
    let body = CreateTransactionBody {
        transaction_index,
        user_id: user_id.to_string(),
        signature,
    };
    let client = reqwest::Client::new();
    let path = format!("{}/transactions", base_url);

    client
        .post(path)
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<Transaction>()
        .await
        .unwrap()
}
