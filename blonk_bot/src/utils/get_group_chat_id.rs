use std::env;
use teloxide::types::ChatId;

pub fn get_group_chat_id() -> ChatId {
    let group_chat_id = env::var("GROUP_CHAT_ID").unwrap();

    ChatId(group_chat_id.parse::<i64>().unwrap())
}
