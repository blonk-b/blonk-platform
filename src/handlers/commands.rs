use teloxide::{prelude::*, utils::command::BotCommands};
use crate::collections::{ MyDialogue, HandlerResult, State, Command };

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult  {
    bot.send_message(msg.chat.id, format!("Hello, share with me a Blink URL to start!")).await?;
    dialogue.update(State::ReceiveBlinkUrl).await?;
    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> HandlerResult  {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the Blink request.").await?;
    dialogue.exit().await?;
    Ok(())
}