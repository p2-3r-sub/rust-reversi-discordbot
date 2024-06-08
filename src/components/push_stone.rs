use serenity::all::{
    ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    UserId,
};

use crate::global_data::GlobalReversiStats;
use crate::reversi::reversi::Stone;

pub async fn run(ctx: &Context, interaction: &ComponentInteraction) {
    let inter_userid = interaction.user.id;
    let inter_channelid = interaction.channel_id.get();

    let is_contains_key = {
        let data = ctx.data.read().await;

        let reversi_stats = data
            .get::<GlobalReversiStats>()
            .expect("Expected GlobalReversiStats in TypeMap.");
        let reversi_stats = reversi_stats.lock().await;

        reversi_stats.contains_key(&inter_channelid)
    };

    if !(is_contains_key) {
        return;
    }

    let check_userid = {
        let data = ctx.data.read().await;

        let reversi_stats = data
            .get::<GlobalReversiStats>()
            .expect("Expected GlobalReversiStats in TypeMap.");
        let reversi_stats = reversi_stats.lock().await;

        let channel_stats = reversi_stats.get(&inter_channelid).unwrap();

        match channel_stats.reversi.turn_stone {
            Stone::BLACK => channel_stats.black_user.as_ref().unwrap().id,
            Stone::WHITE => channel_stats.white_user.as_ref().unwrap().id,
            Stone::NONE => panic!("NoneError"),
        }
    };

    if inter_userid != check_userid {
        cant_notice(ctx, interaction, "手番ではないため置けません。").await;
        return;
    }

    let (push_row, push_column) = {
        let data = ctx.data.read().await;

        let reversi_stats = data
            .get::<GlobalReversiStats>()
            .expect("Expected GlobalReversiStats in TypeMap.");
        let reversi_stats = reversi_stats.lock().await;

        let channel_stats = reversi_stats.get(&inter_channelid).unwrap();

        // 今のリバーシのターンの人の指定した行列を取り出す
        let (alphabet, number) = {
            let black_user = channel_stats.black_user.as_ref().unwrap();
            let white_user = channel_stats.white_user.as_ref().unwrap();

            match channel_stats.reversi.turn_stone {
                Stone::BLACK => (
                    black_user.choiced_alphabet.clone(),
                    black_user.choiced_number.clone(),
                ),
                Stone::WHITE => (
                    white_user.choiced_alphabet.clone(),
                    white_user.choiced_number.clone(),
                ),
                Stone::NONE => panic!("NoneError"),
            }
        };

        if let (Some(alphabet), Some(number)) = (alphabet, number) {
            ("ABCDEFGH".find(&alphabet), "12345678".find(&number))
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

        let reversi_stats = data
            .get_mut::<GlobalReversiStats>()
            .expect("Expected GlobalReversiStats in TypeMap.");
        let mut reversi_stats = reversi_stats.lock().await;

        let channel_id_u64 = interaction.channel_id.get();
        let channel_stats = reversi_stats.get_mut(&channel_id_u64).unwrap();
        let reversi = &mut channel_stats.reversi;

        match reversi.do_place(push_row as i32, push_column as i32, reversi.turn_stone) {
            Ok(_) => (),
            Err(_) => {
                cant_notice(ctx, interaction, "そのマスには置けません。").await;
                return;
            }
        }

        if reversi.is_game_end() {
            let board = reversi.print_board();

            let mut black_count = 0;
            let mut white_count = 0;

            for row in 0..8 {
                for column in 0..8 {
                    match reversi.board[row][column] {
                        Stone::BLACK => black_count += 1,
                        Stone::WHITE => white_count += 1,
                        Stone::NONE => (),
                    }
                }
            }

            let winner = if black_count > white_count {
                format!(
                    "Black: {}",
                    get_username(ctx, &channel_stats.black_user.as_ref().unwrap().id).await
                )
            } else if black_count < white_count {
                format!(
                    "White: {}",
                    get_username(ctx, &channel_stats.white_user.as_ref().unwrap().id).await
                )
            } else {
                "Draw".to_string()
            };

            let content = "試合が終了しました。".to_string();
            let winnerinfo = format!(
                "Black: {}\nWhite: {}\nWinner: {}",
                black_count, white_count, winner
            );

            let data = CreateInteractionResponseMessage::new()
                .content(format!("{}\n{}\n{}", content, board, winnerinfo))
                .components(vec![]);

            let builder = CreateInteractionResponse::UpdateMessage(data);
            if let Err(why) = interaction.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }

            reversi_stats.remove(&channel_id_u64);
            return;
        }

        reversi.switch_turn();
        let mut board = reversi.print_board();

        let mut content = String::new();

        if !(reversi.player_can_place(reversi.turn_stone)) {
            reversi.switch_turn();
            board = reversi.print_board();
            content =
                "置けるマスがなかったためもう一度同じ人のターンです。\n".to_string() + &content;
        }

        let turn_info = match reversi.turn_stone {
            Stone::BLACK => format!(
                "現在 🔵 : {} の番です。",
                get_username(ctx, &channel_stats.black_user.as_ref().unwrap().id).await
            ),
            Stone::WHITE => format!(
                "現在 ⚪ : {} の番です。",
                get_username(ctx, &channel_stats.white_user.as_ref().unwrap().id).await
            ),
            Stone::NONE => panic!("NoneError"),
        };

        content += &turn_info;

        let data = CreateInteractionResponseMessage::new().content(content + "\n\n" + &board);

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
