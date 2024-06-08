use std::borrow::Cow;

use serenity::all::{
    ComponentInteraction, Context, CreateAttachment, CreateInteractionResponse,
    CreateInteractionResponseMessage, UserId,
};

use crate::{
    global_data::GlobalQuantumGomokuStats,
    quantum_gomoku::{gen_image::gen_quantum_board_image, gomoku::Stone},
};

pub async fn run(ctx: &Context, interaction: &ComponentInteraction) {
    let inter_userid = interaction.user.id;
    let inter_channelid = interaction.channel_id.get();

    let is_contains_key = {
        let data = ctx.data.read().await;

        let gomoku_stats = data
            .get::<GlobalQuantumGomokuStats>()
            .expect("Expected GlobalQuantumGomokuStats in TypeMap.");
        let gomoku_stats = gomoku_stats.lock().await;

        gomoku_stats.contains_key(&inter_channelid)
    };

    if !(is_contains_key) {
        return;
    }

    let check_userid = {
        let data = ctx.data.read().await;

        let gomoku_stats = data
            .get::<GlobalQuantumGomokuStats>()
            .expect("Expected GlobalQuantumGomokuStats in TypeMap.");
        let gomoku_stats = gomoku_stats.lock().await;

        let channel_stats = gomoku_stats.get(&inter_channelid).unwrap();

        match channel_stats.gomoku.current_turn {
            Stone::Black90 => channel_stats.black_user.as_ref().unwrap().id,
            Stone::Black70 => channel_stats.black_user.as_ref().unwrap().id,
            Stone::White90 => channel_stats.white_user.as_ref().unwrap().id,
            Stone::White70 => channel_stats.white_user.as_ref().unwrap().id,
            Stone::None => panic!("NoneError"),
        }
    };

    if inter_userid != check_userid {
        cant_notice(ctx, interaction, "手番ではないため置けません。").await;
        return;
    }

    let (push_row, push_column) = {
        let data = ctx.data.read().await;

        let gomoku_stats = data
            .get::<GlobalQuantumGomokuStats>()
            .expect("Expected GlobalQuantumGomokuStats in TypeMap.");
        let gomoku_stats = gomoku_stats.lock().await;

        let channel_stats = gomoku_stats.get(&inter_channelid).unwrap();

        // 今のリバーシのターンの人の指定した行列を取り出す
        let (alphabet, number) = {
            let black_user = channel_stats.black_user.as_ref().unwrap();
            let white_user = channel_stats.white_user.as_ref().unwrap();

            match channel_stats.gomoku.current_turn {
                Stone::Black90 => (
                    black_user.choiced_alphabet.clone(),
                    black_user.choiced_number.clone(),
                ),
                Stone::Black70 => (
                    black_user.choiced_alphabet.clone(),
                    black_user.choiced_number.clone(),
                ),
                Stone::White90 => (
                    white_user.choiced_alphabet.clone(),
                    white_user.choiced_number.clone(),
                ),
                Stone::White70 => (
                    white_user.choiced_alphabet.clone(),
                    white_user.choiced_number.clone(),
                ),
                Stone::None => panic!("NoneError"),
            }
        };

        if let (Some(alphabet), Some(number)) = (alphabet, number) {
            (
                "ABCDEFGHIJKLMNO".find(&alphabet),
                Some((number.parse::<usize>().unwrap()) - 1),
            )
        } else {
            (None, None)
        }
    };

    let (push_row, push_column) = match (push_row, push_column) {
        (Some(row), Some(column)) => (row, column),
        _ => {
            cant_notice(ctx, interaction, "行と列を選択してください。").await;
            return;
        }
    };

    {
        let mut data = ctx.data.write().await;

        let gomoku_stats = data
            .get_mut::<GlobalQuantumGomokuStats>()
            .expect("Expected GlobalQuantumGomokuStats in TypeMap.");
        let mut gomoku_stats = gomoku_stats.lock().await;

        let channel_id_u64 = interaction.channel_id.get();
        let channel_stats = gomoku_stats.get_mut(&channel_id_u64).unwrap();
        let gomoku = &mut channel_stats.gomoku;

        match gomoku.do_place(push_row, push_column) {
            Ok(_) => (),
            Err(_) => {
                cant_notice(ctx, interaction, "そのマスには置けません。").await;
                return;
            }
        }

        gomoku.switch_turn();

        let turn_info = {
            let black_user_name =
                get_username(ctx, &channel_stats.black_user.as_ref().unwrap().id).await;
            let white_user_name =
                get_username(ctx, &channel_stats.white_user.as_ref().unwrap().id).await;

            match gomoku.current_turn {
                Stone::Black90 => format!(
                    "現在 🔵 : {} の番です。\n石: 90%黒 - 10%白",
                    black_user_name
                ),
                Stone::Black70 => format!(
                    "現在 🔵 : {} の番です。\n石: 70%黒 - 30%白",
                    black_user_name
                ),
                Stone::White90 => format!(
                    "現在 ⚪ : {} の番です。\n石: 90%白 - 10%黒",
                    white_user_name
                ),
                Stone::White70 => format!(
                    "現在 ⚪ : {} の番です。\n石: 70%白 - 30%黒",
                    white_user_name
                ),

                Stone::None => panic!("NoneError"),
            }
        };

        let img_vec = gen_quantum_board_image(gomoku.board).await;
        let data = CreateInteractionResponseMessage::new()
            .content(turn_info)
            .add_file(CreateAttachment::bytes(
                Cow::from(img_vec),
                "board.png".to_string(),
            ));

        let builder = CreateInteractionResponse::UpdateMessage(data);
        if let Err(why) = interaction.create_response(&ctx.http, builder).await {
            println!("Cannot respond to slash command: {why}");
        }
    }
}

async fn cant_notice(ctx: &Context, interaction: &ComponentInteraction, content: &str) {
    let data = CreateInteractionResponseMessage::new()
        .content(content)
        .ephemeral(true);

    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = interaction.create_response(&ctx.http, builder).await {
        if why.to_string() == "Cannot send an empty message" {
            return;
        }
        println!("Cannot respond to slash command: {why}");
    }
}

async fn get_username(ctx: &Context, user_id: &UserId) -> String {
    match user_id.to_user(&ctx.http).await {
        Ok(user) => user.name,
        Err(_) => "None".to_string(),
    }
}
