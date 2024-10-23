use collections::{Command, Config, Handler, JoinStorage};
use dptree::{case, deps};
use std::path::PathBuf;
use std::sync::Arc;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::{
    dispatching::dialogue::{self, ErasedStorage, InMemStorage, SqliteStorage, Storage},
    prelude::*,
};

mod actions;
mod collections;
mod commands;
mod handlers;
mod instructions;
mod requests;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    let config = Config {
        channel_id: Some(-4594739971),
        storage_path: Some(PathBuf::from("db.sqlite")),
    };

    let storage: JoinStorage = if let Some(_storage_path) = config.storage_path.clone() {
        SqliteStorage::open("db.sqlite", Json)
            .await
            .unwrap()
            .erase()
    } else {
        InMemStorage::new().erase()
    };

    let handler = dialogue::enter::<Update, ErasedStorage<Handler>, Handler, _>()
        .branch(Update::filter_callback_query().branch(
            case![Handler::InternalAction { data }].endpoint(handlers::handle_internal_action),
        ))
        .branch(Update::filter_callback_query().endpoint(handlers::handle_external_action))
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, ErasedStorage<Handler>, Handler>()
                .branch(case![Handler::BlinkUrl].endpoint(handlers::handle_blink_url))
                .branch(case![Handler::Parameters { data }].endpoint(handlers::handle_parameters)),
        )
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Help].endpoint(commands::help))
                .branch(case![Command::Cancel].endpoint(commands::cancel)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(deps![storage, Arc::new(config)])
        .default_handler(|_| async move {
            // We ignore any update we don't know
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
