use crate::collections::Action;
use serde::{Deserialize, Serialize};
use teloxide::types::UserId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InternalActionData {
    pub actions: Vec<Action>,
    pub url: String,
    pub action_title: String,
    pub action_description: String,
    pub user_id: UserId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParametersData {
    pub parameter_names: Vec<String>,
    pub parameter_labels: Vec<String>,
    pub url: String,
    pub parameters_number: u8,
    pub parameters_values: Vec<String>,
    pub action_title: String,
    pub action_description: String,
    pub user_id: UserId,
}
#[derive(Clone, Default, Deserialize, Serialize)]
pub enum Handler {
    // Start,
    #[default]
    BlinkUrl,
    InternalAction {
        data: InternalActionData,
    },
    Parameters {
        data: ParametersData,
    },
}
