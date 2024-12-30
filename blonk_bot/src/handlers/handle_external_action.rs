use crate::collections::{ButtonMetadata, HandlerResult, JoinStorage};
use crate::requests::{get_multisig_account, get_transaction_account};
use crate::utils::{get_group_chat_id, get_multisig_pubkey, get_transaction_request_buttons};
use teloxide::{prelude::*, types::InlineKeyboardMarkup};

pub async fn handle_external_action(
    bot: Bot,
    _storage: JoinStorage,
    q: CallbackQuery,
) -> HandlerResult {
    let button_metadata: ButtonMetadata = match q.data.unwrap().try_into() {
        Ok(button_metadata) => button_metadata,
        _ => return Ok(()),
    };

    if let Err(e) = bot
        .answer_callback_query(&q.id)
        .text("Processing request...")
        .show_alert(false)
        .await
    {
        eprintln!("Failed to answer callback query: {}", e);
    }

    let transaction_entry = crate::requests::get_transaction(button_metadata.transaction_id).await;
    let multisig_pubkey = get_multisig_pubkey();

    match button_metadata.value.as_str() {
        "Approve" => {
            crate::actions::approve_transaction(
                multisig_pubkey,
                transaction_entry.transaction_index,
                q.from.id,
            )
            .await
        }
        "Reject" => {
            crate::actions::reject_transaction(
                multisig_pubkey,
                transaction_entry.transaction_index,
                q.from.id,
            )
            .await
        }
        _ => {
            crate::actions::execute_transaction(
                multisig_pubkey,
                transaction_entry.transaction_index,
                q.from.id,
            )
            .await
        }
    };

    let multisig_account = get_multisig_account(multisig_pubkey).await;
    let transaction_account =
        get_transaction_account(multisig_pubkey, transaction_entry.transaction_index).await;

    let buttons = get_transaction_request_buttons(
        transaction_entry.id,
        multisig_account.threshold,
        transaction_account.approved.len().try_into().unwrap(),
        transaction_account.rejected.len().try_into().unwrap(),
        &transaction_account.status,
    );

    let group_chat_id = get_group_chat_id();
    let _ = bot
        .edit_message_reply_markup(group_chat_id, q.message.unwrap().id())
        .reply_markup(InlineKeyboardMarkup::new([buttons]))
        .await;

    Ok(())
}
