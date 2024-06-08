use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateActionRow, CreateButton,
    CreateCommandOption, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, ResolvedValue, UserId,
};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::global_data::GlobalReversiStats;
use crate::reversi::stats::RStats;
use crate::reversi::stats::RStatsUser;

pub async fn run(
    options: &[ResolvedOption<'_>],
    interaction: &CommandInteraction,
    ctx: &Context,
) -> CreateInteractionResponseMessage {
    let inter_data = CreateInteractionResponseMessage::new();

    let rival_user = if let Some(ResolvedOption {
        value: ResolvedValue::User(rival_user, _),
        ..
    }) = options.first()
    {
        rival_user
    } else {
        return inter_data.content("その相手は選択できません。");
    };

    if rival_user.bot {
        return inter_data.content("Botは対戦対手に指定できません。");
    }

    let mut data = ctx.data.write().await;

    let reversi_stats = data
        .get_mut::<GlobalReversiStats>()
        .expect("Expected GlobalReversiStats in TypeMap.");
    let mut reversi_stats = reversi_stats.lock().await;

    let channel_id_u64 = interaction.channel_id.get();

    if reversi_stats.contains_key(&channel_id_u64) {
        return inter_data.content("すでに試合が行われています。");
    }

    reversi_stats.insert(channel_id_u64, RStats::new());
    let channel_stats = reversi_stats.get_mut(&channel_id_u64).unwrap();

    channel_stats.black_user = Some(RStatsUser::new(interaction.user.id));
    channel_stats.white_user = Some(RStatsUser::new(rival_user.id));

    let black_username = get_username(ctx, &interaction.user.id).await;
    let board = channel_stats.reversi.print_board();

    return inter_data
        .content(format!(
            "現在 🔵 : {}の番です。\n\n{}",
            black_username, board
        ))
        .components(components(options, interaction));
}

fn components(_options: &[ResolvedOption], _ctx: &CommandInteraction) -> Vec<CreateActionRow> {
    let kind = CreateSelectMenuKind::String {
        options: {
            let mut vec = vec![];
            for i in "ABCDEFGH".chars().into_iter() {
                vec.push(CreateSelectMenuOption::new(i, i));
            }

            vec
        },
    };
    let select_choice_row =
        CreateSelectMenu::new("choice_alphabet", kind).placeholder("行を指定してください");

    let kind = CreateSelectMenuKind::String {
        options: {
            let mut vec = vec![];
            for i in "12345678".chars().into_iter() {
                vec.push(CreateSelectMenuOption::new(i, i));
            }

            vec
        },
    };
    let select_choice_column =
        CreateSelectMenu::new("choice_number", kind).placeholder("列を指定してください");

    let push_btn = CreateButton::new("push_stone").label("置く");

    let row_0 = CreateActionRow::SelectMenu(select_choice_row);
    let row_1 = CreateActionRow::SelectMenu(select_choice_column);
    let row_2 = CreateActionRow::Buttons(vec![push_btn]);

    return vec![row_0, row_1, row_2];
}

async fn get_username(ctx: &Context, user_id: &UserId) -> String {
    match user_id.to_user(&ctx.http).await {
        Ok(user) => user.name,
        Err(_) => "None".to_string(),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("reversi_start")
        .description("リバーシの試合を開始します。")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "対戦相手を指定します。")
                .required(true),
        )
}
