use teloxide::{
    dispatching::dialogue::ErasedStorage,
    prelude::*,
};
use std::sync::Arc;
use crate::collections::State;

pub type MyDialogue = Dialogue<State, ErasedStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type JoinStorage = Arc<ErasedStorage<State>>;