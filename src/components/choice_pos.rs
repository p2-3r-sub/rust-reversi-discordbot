use crate::global_data::GlobalReversiStats;
use serenity::all::{
    ComponentInteraction, ComponentInteractionDataKind, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::reversi::reversi::Stone;

pub async fn alphabet(ctx: &Context, interaction: &ComponentInteraction) {
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
        send_empty(ctx, interaction).await;
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
        send_empty(ctx, interaction).await;
        return;
    }

    let choiced_val =
        if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
            values.get(0).unwrap().clone()
        } else {
            panic!("Error!");
        };

    {
        let mut data = ctx.data.write().await;

        let reversi_stats = data
            .get_mut::<GlobalReversiStats>()
            .expect("Expected GlobalReversiStats in TypeMap.");
        let mut reversi_stats = reversi_stats.lock().await;

        let channel_id_u64 = interaction.channel_id.get();
        let channel_stats = reversi_stats.get_mut(&channel_id_u64).unwrap();

        match channel_stats.reversi.turn_stone {
            Stone::BLACK => {
                channel_stats.black_user.as_mut().unwrap().choiced_alphabet = Some(choiced_val)
            }
            Stone::WHITE => {
                channel_stats.white_user.as_mut().unwrap().choiced_alphabet = Some(choiced_val)
            }
            Stone::NONE => panic!("NoneError"),
        };
    }

    send_empty(ctx, interaction).await;
}

pub async fn number(ctx: &Context, interaction: &ComponentInteraction) {
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
        send_empty(ctx, interaction).await;
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
        send_empty(ctx, interaction).await;
        return;
    }

    let choiced_val =
        if let ComponentInteractionDataKind::StringSelect { values } = &interaction.data.kind {
            values.get(0).unwrap().clone()
        } else {
            panic!("Error!");
        };

    {
        let mut data = ctx.data.write().await;

        let reversi_stats = data
            .get_mut::<GlobalReversiStats>()
            .expect("Expected GlobalReversiStats in TypeMap.");
        let mut reversi_stats = reversi_stats.lock().await;

        let channel_id_u64 = interaction.channel_id.get();
        let channel_stats = reversi_stats.get_mut(&channel_id_u64).unwrap();

        match channel_stats.reversi.turn_stone {
            Stone::BLACK => {
                channel_stats.black_user.as_mut().unwrap().choiced_number = Some(choiced_val)
            }
            Stone::WHITE => {
                channel_stats.white_user.as_mut().unwrap().choiced_number = Some(choiced_val)
            }
            Stone::NONE => panic!("NoneError"),
        };
    }

    send_empty(ctx, interaction).await;
}

async fn send_empty(ctx: &Context, interaction: &ComponentInteraction) {
    let data = CreateInteractionResponseMessage::new().content("");

    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = interaction.create_response(&ctx.http, builder).await {
        if why.to_string() == "Cannot send an empty message" {
            return;
        }
        println!("Cannot respond to slash command: {why}");
    }
}
