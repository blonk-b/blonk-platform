use crate::collections::{Command, HandlerResult};
use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
