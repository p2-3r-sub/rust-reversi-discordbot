use serenity::all::{CommandInteraction, Context, CreateInteractionResponseMessage};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub async fn run(
    _options: &[ResolvedOption<'_>],
    _interaction: &CommandInteraction,
    _ctx: &Context,
) -> CreateInteractionResponseMessage {
    let inter_data = CreateInteractionResponseMessage::new();

    return inter_data.content("Pong!");
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
