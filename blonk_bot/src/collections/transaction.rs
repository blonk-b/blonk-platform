#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub transaction_index: u32,
    pub user_id: String,
    pub signature: String,
    pub status: u8,
    pub message_id: Option<String>,
}
