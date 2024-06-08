use std::borrow::Cow;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateActionRow, CreateAttachment,
    CreateButton, CreateCommandOption, CreateInteractionResponseMessage, CreateSelectMenu,
    CreateSelectMenuKind, CreateSelectMenuOption, ResolvedValue, UserId,
};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::global_data::GlobalQuantumGomokuStats;
use crate::quantum_gomoku::gen_image::gen_quantum_board_image;
use crate::quantum_gomoku::stats::{QGStats, QGStatsUser};

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
        .get_mut::<GlobalQuantumGomokuStats>()
        .expect("Expected GlobalReversiStats in TypeMap.");
    let mut reversi_stats = reversi_stats.lock().await;

    let channel_id_u64 = interaction.channel_id.get();

    if reversi_stats.contains_key(&channel_id_u64) {
        return inter_data.content("すでに試合が行われています。");
    }

    reversi_stats.insert(channel_id_u64, QGStats::new());
    let channel_stats = reversi_stats.get_mut(&channel_id_u64).unwrap();

    channel_stats.black_user = Some(QGStatsUser::new(interaction.user.id));
    channel_stats.white_user = Some(QGStatsUser::new(rival_user.id));

    let black_username = get_username(ctx, &interaction.user.id).await;

    let img_vec = gen_quantum_board_image(channel_stats.gomoku.board).await;

    let rule_notice = "> ルール・元ネタ: \n> QuizKnock [【理解不能】何色になるか分からない量子で五目並べやってみた【でも楽しそう】](https://www.youtube.com/watch?v=mitAxA3f4U4)";
    return inter_data
        .content(format!(
            "{}\n現在 🔵 : {}の番です。\n石: 90%黒 - 10%白",
            rule_notice, black_username
        ))
        .add_file(CreateAttachment::bytes(
            Cow::from(img_vec),
            "board.png".to_string(),
        ))
        .components(components(options, interaction));
}

async fn get_username(ctx: &Context, user_id: &UserId) -> String {
    match user_id.to_user(&ctx.http).await {
        Ok(user) => user.name,
        Err(_) => "None".to_string(),
    }
}

fn components(_options: &[ResolvedOption], _ctx: &CommandInteraction) -> Vec<CreateActionRow> {
    let kind = CreateSelectMenuKind::String {
        options: {
            let mut vec = vec![];
            for i in "ABCDEFGHIJKLMNO".chars().into_iter() {
                vec.push(CreateSelectMenuOption::new(i, i));
            }

            vec
        },
    };
    let select_choice_row =
        CreateSelectMenu::new("qgomoku_choice_alphabet", kind).placeholder("列を指定してください");

    let kind = CreateSelectMenuKind::String {
        options: {
            let mut vec = vec![];
            for i in (1..=15).into_iter() {
                let string = i.to_string();
                vec.push(CreateSelectMenuOption::new(string.clone(), string.clone()));
            }

            vec
        },
    };

    let select_choice_column =
        CreateSelectMenu::new("qgomoku_choice_number", kind).placeholder("行を指定してください");

    let push_btn = CreateButton::new("qgomoku_push_stone").label("置く");
    let push_observe_btn = CreateButton::new("qgomoku_push_stone_observe").label("置いて観測する");

    let row_0 = CreateActionRow::SelectMenu(select_choice_row);
    let row_1 = CreateActionRow::SelectMenu(select_choice_column);
    let row_2 = CreateActionRow::Buttons(vec![push_btn, push_observe_btn]);

    return vec![row_0, row_1, row_2];
}

pub fn register() -> CreateCommand {
    CreateCommand::new("q_gomoku_start")
        .description("量子五目並べをスタートします。")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "対戦相手を指定します。")
                .required(true),
        )
}
