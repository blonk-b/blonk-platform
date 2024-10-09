use crate::actions::{ create_transaction, execute_transaction };
use crate::collections::{
    HandlerResult, JoinStorage, MyDialogue, ReceiveActionData, ReceiveParametersData, State,
    TransactionData,
};
use crate::queries::{generate_transaction, get_blink_metadata};
use crate::tools::get_url_root;
use crate::utils::RPC;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, message::Message as TransactionMessage, signer::keypair::Keypair,
    transaction::Transaction,
};
use solana_sdk::signer::Signer;

use std::env;
use teloxide::{
    prelude::*,
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};
use url::form_urlencoded;

const GROUP_CHAT_ID: ChatId = ChatId(-4594739971);

pub async fn receive_blink_url(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(url) => {
            let response = get_blink_metadata(&url.to_string()).await;

            match response {
                Ok(res) => {
                    let links = res.links;

                    match links {
                        Some(links_res) => {
                            let actions = links_res.actions;
                            let action_names: Vec<String> =
                                actions.iter().map(|a| a.label.clone()).collect();
                            let action_buttons = action_names
                                .iter()
                                .map(|action| InlineKeyboardButton::callback(action, action));

                            let template = format!(
                                "<b>{}</b> \n\n{} \n\n{}\n\n Choose an action to perform:",
                                res.title, res.description, res.icon
                            );

                            bot.send_message(msg.chat.id, template)
                                .parse_mode(ParseMode::Html)
                                .reply_markup(InlineKeyboardMarkup::new([action_buttons]))
                                .await?;

                            let actions_data = ReceiveActionData {
                                actions,
                                url: url.to_string(),
                                action_title: res.title,
                                action_description: res.description,
                            };

                            dialogue
                                .update(State::ReceiveAction { data: actions_data })
                                .await?;
                        }
                        None => {
                            let transaction_response =
                                generate_transaction(&url.to_string()).await?;

                            let template = format!(
                                "<b>{}</b> \n\n{} \n\n{} \n\n{}",
                                res.title,
                                res.description,
                                res.icon,
                                transaction_response.transaction
                            );

                            bot.send_message(msg.chat.id, template)
                                .parse_mode(ParseMode::Html)
                                .await?;

                            dialogue.exit().await?;
                        }
                    }
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error getting blink: {}", e))
                        .await?;
                }
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Please, share with me a valid Blink URL")
                .await?;
        }
    }

    Ok(())
}

pub async fn receive_action(
    bot: Bot,
    dialogue: MyDialogue,
    data: ReceiveActionData,
    q: CallbackQuery,
) -> HandlerResult {
    if let Some(action_name) = &q.data {
        let action = data
            .actions
            .iter()
            .find(|&a| a.label == action_name.to_string());

        match action {
            Some(res) => {
                let parameters = res.parameters.clone();

                match parameters {
                    Some(parameters_res) => {
                        if parameters_res.len() == 0 {
                            let transaction_response = generate_transaction(&data.url).await?;

                            bot.send_message(
                                dialogue.chat_id(),
                                format!("{}", transaction_response.transaction),
                            )
                            .parse_mode(ParseMode::Html)
                            .await?;

                            dialogue.exit().await?;
                        } else {
                            let parameter_names: Vec<String> =
                                parameters_res.iter().map(|p| p.name.clone()).collect();
                            let parameter_labels: Vec<String> =
                                parameters_res.iter().map(|p| p.label.clone()).collect();
                            let mut result_parameters = String::new();

                            for (index, parameter) in parameters_res.iter().enumerate() {
                                match parameter.required {
                                    Some(is_required) => {
                                        result_parameters.push_str(&format!(
                                            "{} - {} ({})\n\n",
                                            index + 1,
                                            parameter.label,
                                            if is_required { "Required" } else { "Optional" }
                                        ));
                                    }
                                    None => {
                                        result_parameters.push_str(&format!(
                                            "{} - {} (Optional)\n\n",
                                            index + 1,
                                            parameter.label
                                        ));
                                    }
                                }
                            }

                            bot.send_message(dialogue.chat_id(), format!(
                                "The action <b>{}</b> has the following params: \n\n{}Please enter the values as they are requested.", 
                                action_name, result_parameters
                                )
                            )
                            .parse_mode(ParseMode::Html)
                            .await?;

                            bot.send_message(
                                dialogue.chat_id(),
                                format!("{}", parameter_labels[0]),
                            )
                            .parse_mode(ParseMode::Html)
                            .await?;

                            let parameters_values: Vec<String> = Vec::new();

                            let parameters_data = ReceiveParametersData {
                                parameter_names,
                                parameter_labels,
                                url: format!("{}{}", get_url_root(&data.url).unwrap(), res.href),
                                parameters_number: 1,
                                parameters_values,
                                action_title: data.action_title,
                                action_description: data.action_description,
                            };

                            dialogue
                                .update(State::ReceiveParameters {
                                    data: parameters_data,
                                })
                                .await?;
                        }
                    }
                    None => {
                        let template = format!(
                            "<b>New transaction request:</b> {}\n\n{}\n\n",
                            data.action_title, data.action_description
                        );

                        let transaction_index = create_transaction(&data.url).await?;

                        println!("TRANSACTION INDEX: {}", transaction_index);
                        let threshold: u8 = 2;

                        let buttons: Vec<TransactionData> = vec![
                            TransactionData {
                                chat_id: dialogue.chat_id(),
                                label: format!("Approve {}/{}", 1, threshold),
                                value: format!("Approve"),
                                authority_account: format!("TEST"),
                                transaction_index,
                                threshold,
                                current_threshold: 1,
                                reject_count: 0,
                                blink_name: format!("Transaction Nº{}", 1),
                            },
                            TransactionData {
                                chat_id: dialogue.chat_id(),
                                label: format!("Reject {}/{}", 0, threshold),
                                value: format!("Reject"),
                                authority_account: format!("TEST"),
                                transaction_index,
                                threshold,
                                current_threshold: 1,
                                reject_count: 0,
                                blink_name: format!("Transaction Nº{}", 1),
                            },
                        ];

                        let transaction_buttons = buttons.iter().map(|button| {
                            InlineKeyboardButton::callback(button.label.clone(), button)
                        });

                        bot.send_message(GROUP_CHAT_ID, format!("{}", template))
                            .parse_mode(ParseMode::Html)
                            .reply_markup(InlineKeyboardMarkup::new([transaction_buttons]))
                            .await?;

                        bot.send_message(dialogue.chat_id(), format!("Transaction sent!"))
                            .await?;

                        dialogue.exit().await?;
                    }
                }
            }
            None => {
                bot.send_message(dialogue.chat_id(), "Send me a valid action.")
                    .await?;
            }
        }
    }

    Ok(())
}

pub async fn receive_parameters(
    bot: Bot,
    dialogue: MyDialogue,
    data: ReceiveParametersData,
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
                    format!("{}", data.parameter_labels[current_param_number]),
                )
                .parse_mode(ParseMode::Html)
                .await?;

                let parameters_data = ReceiveParametersData {
                    parameter_names: data.parameter_names,
                    parameter_labels: data.parameter_labels,
                    url: data.url,
                    parameters_number: data.parameters_number + 1,
                    parameters_values,
                    action_title: data.action_title,
                    action_description: data.action_description,
                };

                dialogue
                    .update(State::ReceiveParameters {
                        data: parameters_data,
                    })
                    .await?;
            } else {
                let mut request_url = data.url;

                for (index, parameter) in data.parameter_names.iter().enumerate() {
                    let item = format!("{{{}}}", parameter);
                    let value: String =
                        form_urlencoded::byte_serialize(&parameters_values[index].as_bytes())
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

                let transaction_index = create_transaction(&request_url).await?;
                println!("\n\nTRANSACTION INDEX: {}\n\n", transaction_index);

                let threshold: u8 = 2;

                let buttons: Vec<TransactionData> = vec![
                    TransactionData {
                        chat_id: msg.chat.id,
                        label: format!("Approve {}/{}", 1, threshold),
                        value: format!("Approve"),
                        authority_account: format!("TEST"),
                        transaction_index,
                        threshold,
                        current_threshold: 1,
                        reject_count: 0,
                        blink_name: format!("Transaction Nº{}", transaction_index),
                    },
                    TransactionData {
                        chat_id: msg.chat.id,
                        label: format!("Reject {}/{}", 0, threshold),
                        value: format!("Reject"),
                        authority_account: format!("TEST"),
                        transaction_index,
                        threshold,
                        current_threshold: 1,
                        reject_count: 0,
                        blink_name: format!("Transaction Nº{}", transaction_index),
                    },
                ];

                let template = format!(
                    "<b>New transaction request:</b> {}\n\n{}\n\n{}\nTransaction Nº{}",
                    data.action_title, data.action_description, group_parameters, transaction_index
                );

                let transaction_buttons = buttons
                    .iter()
                    .map(|button| InlineKeyboardButton::callback(button.label.clone(), button));

                bot.send_message(GROUP_CHAT_ID, format!("{}", template))
                    .parse_mode(ParseMode::Html)
                    .reply_markup(InlineKeyboardMarkup::new([transaction_buttons]))
                    .await?;

                bot.send_message(msg.chat.id, format!("Transaction sent!"))
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

pub async fn approve_transaction(
    bot: Bot,
    storage: JoinStorage,
    q: CallbackQuery,
) -> HandlerResult {
    let data = match q.data {
        Some(data) => data,
        None => return Ok(()),
    };

    let transaction_data: TransactionData = match data.try_into() {
        Ok(review) => review,
        Err(error) => {
            println!("error: {:?}", error);
            return Ok(());
        }
    };

    if transaction_data.current_threshold == transaction_data.threshold {
        
        // let _ = execute_transaction(transaction_data.transaction_index);

        bot.send_message(
            GROUP_CHAT_ID,
            format!(
                "<b>{}</b>\n\nTransaction approved! {}/{}",
                transaction_data.blink_name,
                transaction_data.current_threshold,
                transaction_data.threshold
            ),
        )
        .parse_mode(ParseMode::Html)
        .await?;

        let _ = storage.remove_dialogue(transaction_data.chat_id).await;

    } else if transaction_data.reject_count == transaction_data.threshold {
        bot.send_message(
            GROUP_CHAT_ID,
            format!(
                "<b>{}</b>\n\nTransaction rejected! {}/{}",
                transaction_data.blink_name,
                transaction_data.reject_count,
                transaction_data.threshold
            ),
        )
        .parse_mode(ParseMode::Html)
        .await?;

        let _ = storage.remove_dialogue(transaction_data.chat_id).await;
    } else {
        let solana_client = RpcClient::new(RPC.to_string());

        let secret_key = env::var("PAYER_PRIVATE_KEY").unwrap();
        let signer = Keypair::from_base58_string(&secret_key);
        let signer_ref = &signer;
    
        let blockhash = solana_client.get_latest_blockhash().await.unwrap();

        let tx = if transaction_data.value == "Approve" {
            let instruction: Instruction =
                crate::queries::approve_transaction(transaction_data.transaction_index).await;
            let transaction_message =
                TransactionMessage::new(&[instruction], Some(&signer.pubkey()));
            Transaction::new(&[signer_ref], transaction_message, blockhash)
        } else {
            let instruction: Instruction =
                crate::queries::refect_transaction(transaction_data.transaction_index).await;
            let transaction_message =
                TransactionMessage::new(&[instruction], Some(&signer.pubkey()));
            Transaction::new(&[signer_ref], transaction_message, blockhash)
        };

        let transaction_res = solana_client.send_and_confirm_transaction(&tx).await;

        match transaction_res {
            Ok(res) => {
                println!("VOTE SIGNATURE {}: \n\n{}", transaction_data.value, res);

                let current_threshold = if transaction_data.value == "Approve"
                    && transaction_data.current_threshold < transaction_data.threshold
                {
                    transaction_data.current_threshold + 1
                } else if transaction_data.value == "Reject" && transaction_data.current_threshold != 0 {
                    transaction_data.current_threshold - 1
                } 
                else {
                    transaction_data.current_threshold
                };


                let reject_count = if transaction_data.value == "Reject"
                    && transaction_data.reject_count < transaction_data.threshold
                {
                    transaction_data.reject_count + 1
                } else if transaction_data.value == "Approve" && transaction_data.reject_count != 0 {
                    transaction_data.reject_count - 1
                } else {
                    transaction_data.reject_count
                };

                let buttons: Vec<TransactionData> = vec![
                    TransactionData {
                        chat_id: transaction_data.chat_id,
                        label: format!(
                            "Approve {}/{}",
                            current_threshold, transaction_data.threshold
                        ),
                        value: format!("Approve"),
                        authority_account: transaction_data.authority_account.clone(),
                        transaction_index: transaction_data.transaction_index,
                        threshold: transaction_data.threshold,
                        current_threshold,
                        reject_count,
                        blink_name: transaction_data.blink_name.clone(),
                    },
                    TransactionData {
                        chat_id: transaction_data.chat_id,
                        label: format!("Reject {}/{}", reject_count, transaction_data.threshold),
                        value: format!("Reject"),
                        authority_account: transaction_data.authority_account.clone(),
                        transaction_index: transaction_data.transaction_index,
                        threshold: transaction_data.threshold,
                        current_threshold,
                        reject_count,
                        blink_name: transaction_data.blink_name.clone(),
                    },
                ];

                let transaction_buttons = buttons
                    .iter()
                    .map(|button| InlineKeyboardButton::callback(button.label.clone(), button));

                bot.send_message(
                    GROUP_CHAT_ID,
                    format!("<b>{}</b>", transaction_data.blink_name),
                )
                .parse_mode(ParseMode::Html)
                .reply_markup(InlineKeyboardMarkup::new([transaction_buttons]))
                .await?;
            }
            Err(e) => {
                println!("VOTE ERROR {}: \n\n{}", transaction_data.value, e);

                bot.send_message(GROUP_CHAT_ID, format!("Error sending vote, try again!"))
                    .parse_mode(ParseMode::Html)
                    .await?;
            }
        }
    }

    Ok(())
}
