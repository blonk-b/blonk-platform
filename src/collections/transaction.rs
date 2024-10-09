use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlinkTransaction {
    pub transaction: String,
    pub message: Option<String>,
}