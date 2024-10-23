use crate::collections::Transaction;
use serde::{Deserialize, Serialize};
use std::env;
use teloxide::types::MessageId;

#[derive(Serialize, Deserialize)]
pub struct UpdateTransactionBody {
    pub message_id: String,
}

pub async fn update_transaction(id: i64, message_id: MessageId) -> Transaction {
    let base_url = env::var("API_BASE_URL").unwrap();
    let body = UpdateTransactionBody {
        message_id: message_id.to_string(),
    };
    let client = reqwest::Client::new();
    let path = format!("{}/transactions/{}", base_url, id);

    client
        .patch(path)
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<Transaction>()
        .await
        .unwrap()
}
