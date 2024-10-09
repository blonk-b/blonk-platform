use serde::{Deserialize, Serialize};
use crate::collections::Action;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceiveActionData {
    pub actions: Vec<Action>,
    pub url: String,
    pub action_title: String,
    pub action_description: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceiveParametersData {
    pub parameter_names: Vec<String>,
    pub parameter_labels: Vec<String>,
    pub url: String,
    pub parameters_number: u8,
    pub parameters_values: Vec<String>,
    pub action_title: String,
    pub action_description: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApproveTransactionData {
    pub threshold: u8
}