use squads_mpl::state::MsTransactionStatus;
use teloxide::types::InlineKeyboardButton;

use crate::collections::ButtonMetadata;

pub fn get_transaction_request_buttons(
    transaction_id: i64,
    threshold: u16,
    approved: u16,
    rejected: u16,
    status: &MsTransactionStatus,
) -> Vec<InlineKeyboardButton> {
    match status {
        MsTransactionStatus::Active => {
            vec![
                InlineKeyboardButton::callback(
                    format!("Approve {}/{}", approved, threshold),
                    &ButtonMetadata {
                        transaction_id,
                        value: "Approve".to_string(),
                    },
                ),
                InlineKeyboardButton::callback(
                    format!("Reject {}/{}", rejected, threshold),
                    &ButtonMetadata {
                        transaction_id,
                        value: "Reject".to_string(),
                    },
                ),
            ]
        }
        MsTransactionStatus::ExecuteReady => {
            vec![InlineKeyboardButton::callback(
                "Execute".to_string(),
                &ButtonMetadata {
                    transaction_id,
                    value: "Execute".to_string(),
                },
            )]
        }
        _ => {
            vec![]
        }
    }
}
