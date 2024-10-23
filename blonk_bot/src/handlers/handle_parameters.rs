use crate::{
    collections::{Handler, HandlerResult, MyDialogue, ParametersData},
    requests::{get_multisig_account, get_transaction_account},
    utils::{
        get_group_chat_id, get_multisig_pubkey, get_transaction_request_buttons,
        get_transaction_request_message,
    },
};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};
use url::form_urlencoded;

pub async fn handle_parameters(
    bot: Bot,
    dialogue: MyDialogue,
    data: ParametersData,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(parameter_value) => {
            let mut parameters_values = data.parameters_values;
            parameters_values.push(parameter_value.to_string());

            let current_param_number = usize::from(data.parameters_number);

            if current_param_number < data.parameter_names.len() {
                bot.send_message(
                    dialogue.chat_id(),
                    data.parameter_labels[current_param_number].to_string(),
                )
                .parse_mode(ParseMode::Html)
                .await?;

                let parameters_data = ParametersData {
                    parameter_names: data.parameter_names,
                    parameter_labels: data.parameter_labels,
                    url: data.url,
                    parameters_number: data.parameters_number + 1,
                    parameters_values,
                    action_title: data.action_title,
                    action_description: data.action_description,
                    user_id: data.user_id,
                };

                dialogue
                    .update(Handler::Parameters {
                        data: parameters_data,
                    })
                    .await?;
            } else {
                let mut request_url = data.url;

                for (index, parameter) in data.parameter_names.iter().enumerate() {
                    let item = format!("{{{}}}", parameter);
                    let value: String =
                        form_urlencoded::byte_serialize(parameters_values[index].as_bytes())
                            .collect();

                    request_url = request_url.replace(&item, &value);
                }

                let mut group_parameters = String::new();

                for (index, parameter) in data.parameter_labels.iter().enumerate() {
                    group_parameters.push_str(&format!(
                        "<b>{}:</b> {}\n\n",
                        parameter, parameters_values[index]
                    ));
                }

                let multsig_pubkey = get_multisig_pubkey();
                let transaction_entry =
                    crate::actions::create_transaction(&request_url, multsig_pubkey, data.user_id)
                        .await;

                let multisig_account = get_multisig_account(multsig_pubkey).await;
                let threshold = multisig_account.threshold;

                let transaction_account =
                    get_transaction_account(multsig_pubkey, transaction_entry.transaction_index)
                        .await;

                let template = get_transaction_request_message(
                    data.action_title,
                    data.action_description,
                    Some(group_parameters),
                    transaction_entry.transaction_index,
                );

                let buttons = get_transaction_request_buttons(
                    transaction_entry.id,
                    threshold,
                    1,
                    0,
                    &transaction_account.status,
                );

                let group_chat_id = get_group_chat_id();
                let group_message = bot
                    .send_message(group_chat_id, template)
                    .parse_mode(ParseMode::Html)
                    .reply_markup(InlineKeyboardMarkup::new([buttons]))
                    .await?;

                crate::requests::update_transaction(transaction_entry.id, group_message.id).await;

                bot.send_message(msg.chat.id, "Transaction sent!".to_string())
                    .await?;

                dialogue.exit().await?;
            }
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
