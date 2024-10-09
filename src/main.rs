use std::path::PathBuf;

use teloxide::{
    dispatching::dialogue::{ self, InMemStorage, Storage, SqliteStorage, ErasedStorage },
    prelude::*,
};
use teloxide::dispatching::dialogue::serializer::Json;
use collections::{ Command, State, JoinStorage, Config };
use handlers::{ 
    commands::{ cancel, help }, 
    steps::{ 
        receive_action, 
        receive_blink_url, 
        receive_parameters,
        approve_transaction
    } 
};
use dptree::{case, deps};
use std::sync::Arc;

mod collections;
mod queries;
mod handlers;
mod tools;
mod actions;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    let config = Config {
        channel_id: Some(-4594739971),
        storage_path: Some(PathBuf::from("db.sqlite"))
    };

    let storage: JoinStorage = if let Some(_storage_path) = config.storage_path.clone() {
        SqliteStorage::open(
            "db.sqlite",
            Json,
        )
        .await
        .unwrap()
        .erase()
    } else {
        InMemStorage::new().erase()
    };

    let handler = dialogue::enter::<Update, ErasedStorage<State>, State, _>()
        .branch(
            Update::filter_callback_query()
                .branch(case![State::ReceiveAction { data }].endpoint(receive_action))
                .endpoint(approve_transaction) 
        ) 
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                    // .branch(case![Command::Start].endpoint(start))
                    .branch(case![Command::Help].endpoint(help))
                    .branch(case![Command::Cancel].endpoint(cancel))
        )
        .branch(
            Update::filter_message()
            .enter_dialogue::<Message, ErasedStorage<State>, State>()
            .branch(case![State::ReceiveBlinkUrl].endpoint(receive_blink_url))
            .branch(case![State::ReceiveParameters { data }].endpoint(receive_parameters))
        )
    ;

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
