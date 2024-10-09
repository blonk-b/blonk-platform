use std::path::PathBuf;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct Config {
    // pub primary_chat_id: i64,
    pub channel_id: Option<i64>,
    // pub storage_path: Option<PathBuf>,
    pub storage_path: Option<PathBuf>,
}