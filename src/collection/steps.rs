use crate::collections::{ 
    ReceiveActionData, 
    ReceiveParametersData,
    ApproveTransactionData
};

use serde::{ Serialize, Deserialize };

#[derive(Clone, Default, Deserialize, Serialize)]
pub enum State {
    // Start,
    #[default]
    ReceiveBlinkUrl,
    ReceiveAction {
        data: ReceiveActionData
    },
    ReceiveParameters {
        data: ReceiveParametersData
    },
    ApproveTransaction {
        data: ApproveTransactionData
    },
}