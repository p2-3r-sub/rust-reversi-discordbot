use serenity::all::{CommandInteraction, Context, CreateInteractionResponseMessage};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::global_data::GlobalQuantumGomokuStats;

pub async fn run(
    _options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
    ctx: &Context,
) -> CreateInteractionResponseMessage {
    let inter_data = CreateInteractionResponseMessage::new();

    let mut data = ctx.data.write().await;

    let reversi_stats = data
        .get_mut::<GlobalQuantumGomokuStats>()
        .expect("Expected GlobalReversiStats in TypeMap.");
    let mut reversi_stats = reversi_stats.lock().await;
    let channel_id_u64 = interaction.channel_id.get();

    if !(reversi_stats.contains_key(&channel_id_u64)) {
        return inter_data.content("試合は行われていません。");
    }

    reversi_stats.remove(&channel_id_u64);

    return inter_data.content("試合を終了しました。");
}

pub fn register() -> CreateCommand {
    CreateCommand::new("q_gomoku_end").description("量子五目並べを終了します。")
}
