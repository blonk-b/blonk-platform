use crate::collections::Transaction;
use std::env;

pub async fn get_transaction(id: i64) -> Transaction {
    let base_url = env::var("API_BASE_URL").unwrap();
    let client = reqwest::Client::new();
    let path = format!("{}/transactions/{}", base_url, id);

    client
        .get(path)
        .send()
        .await
        .unwrap()
        .json::<Transaction>()
        .await
        .unwrap()
}
