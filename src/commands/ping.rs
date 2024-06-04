use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

#[allow(unused)]
pub fn run(_options: &[ResolvedOption]) -> Option<String> {
    Some("Pong!".to_string())
}

#[allow(unused)]
pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
