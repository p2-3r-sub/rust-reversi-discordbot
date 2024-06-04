use serenity::all::{CommandInteraction, Context, CreateActionRow};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::global_data::GlobalReversiStats;

pub async fn run(
    _options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
    ctx: &Context,
) -> (Option<String>, Option<Vec<CreateActionRow>>) {
    let mut data = ctx.data.write().await;

    let reversi_stats = data
        .get_mut::<GlobalReversiStats>()
        .expect("Expected GlobalReversiStats in TypeMap.");
    let mut reversi_stats = reversi_stats.lock().await;
    let channel_id_u64 = interaction.channel_id.get();

    if !(reversi_stats.contains_key(&channel_id_u64)) {
        return (Some("試合は行われていません。".to_string()), None);
    }

    reversi_stats.remove(&channel_id_u64);

    return (Some("試合を終了しました。".to_string()), None);
}

pub fn register() -> CreateCommand {
    CreateCommand::new("match_end").description("リバーシの試合を終了します。")
}
