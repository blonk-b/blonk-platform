use crate::collections::Handler;
use std::sync::Arc;
use teloxide::{dispatching::dialogue::ErasedStorage, prelude::*};

pub type MyDialogue = Dialogue<Handler, ErasedStorage<Handler>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub type JoinStorage = Arc<ErasedStorage<Handler>>;
