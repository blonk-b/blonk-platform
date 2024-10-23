use crate::collections::{HandlerResult, MyDialogue};
use teloxide::prelude::*;

pub async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the Blink request.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}
