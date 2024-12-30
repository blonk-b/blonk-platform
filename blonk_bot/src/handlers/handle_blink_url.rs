use crate::{
    collections::{Handler, HandlerResult, InternalActionData, MyDialogue},
    requests::{get_blink_metadata, get_blink_transaction},
    utils::get_multisig_pubkey,
};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};
use url::Url;

pub async fn handle_blink_url(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(url) => {
            let response = get_blink_metadata(&url.to_string()).await;
            let parsed_url = Url::parse(url)?;
            let base_url = format!("{}://{}", parsed_url.scheme(), parsed_url.host().unwrap());

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

                            let actions_data = InternalActionData {
                                actions,
                                url: url.to_string(),
                                action_title: res.title,
                                action_description: res.description,
                                user_id: msg.from.clone().unwrap().id,
                                base_url,
                            };

                            dialogue
                                .update(Handler::InternalAction { data: actions_data })
                                .await?;
                        }
                        None => {
                            let multisig_pubkey = get_multisig_pubkey();
                            let transaction_response =
                                get_blink_transaction(multisig_pubkey, &url.to_string()).await?;

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
