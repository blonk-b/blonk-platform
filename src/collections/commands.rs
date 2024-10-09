use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Shows all supported commands")]
    Help,
    #[command(description = "Initialize the flow.")]
    Start,
    #[command(description = "Cancel the Blink request.")]
    Cancel,
}
