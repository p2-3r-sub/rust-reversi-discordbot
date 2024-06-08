#[allow(unused_imports)]
use serenity::all::{ActivityData, Command, Ready};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

mod commands;
mod components;
mod config;
mod global_data;
mod quantum_gomoku;
mod reversi;

use components::{
    choice_pos, push_stone, qgomoku_choice_pos, qgomoku_push_stone, qgomoku_push_stone_observe,
};
use global_data::{GlobalQuantumGomokuStats, GlobalReversiStats};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = &interaction {
            let data = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options(), &command, &ctx).await,
                "reversi_start" => {
                    commands::reversi_start::run(&command.data.options(), &command, &ctx).await
                }

                "reversi_end" => {
                    commands::reversi_end::run(&command.data.options(), &command, &ctx).await
                }

                "q_gomoku_start" => {
                    commands::q_gomoku_start::run(&command.data.options(), &command, &ctx).await
                }

                "q_gomoku_end" => {
                    commands::q_gomoku_end::run(&command.data.options(), &command, &ctx).await
                }

                _ => CreateInteractionResponseMessage::new().content("not implemented"),
            };

            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }

        if let Interaction::Component(interaction) = &interaction {
            let custom_id = &interaction.data.custom_id;

            match custom_id.as_str() {
                "choice_alphabet" => choice_pos::alphabet(&ctx, interaction).await,
                "choice_number" => choice_pos::number(&ctx, interaction).await,
                "push_stone" => push_stone::run(&ctx, interaction).await,

                "qgomoku_choice_alphabet" => qgomoku_choice_pos::alphabet(&ctx, interaction).await,
                "qgomoku_choice_number" => qgomoku_choice_pos::number(&ctx, interaction).await,
                "qgomoku_push_stone" => qgomoku_push_stone::run(&ctx, interaction).await,
                "qgomoku_push_stone_observe" => {
                    qgomoku_push_stone_observe::run(&ctx, interaction).await
                }

                _ => (),
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("login: {}", ready.user.name);

        for i in [
            commands::ping::register(),
            commands::reversi_start::register(),
            commands::reversi_end::register(),
            commands::q_gomoku_start::register(),
            commands::q_gomoku_end::register(),
        ] {
            match Command::create_global_command(&ctx.http, i).await {
                Ok(result) => println!("SetGuildCommand: {}", result.name),
                Err(result) => println!("{}", result),
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = config::get_token("config.json").expect("'config.json' file is not found.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .activity(ActivityData::custom(
            "/reversi_start でリバーシの試合を開始",
        ))
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GlobalReversiStats>(Arc::new(Mutex::new(HashMap::new())));
        data.insert::<GlobalQuantumGomokuStats>(Arc::new(Mutex::new(HashMap::new())));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[tokio::test]
#[ignore]
async fn bot_test() {
    let token =
        config::get_token("config_test.json").expect("'config_test.json' file is not found.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .activity(ActivityData::custom(
            "/reversi_start でリバーシの試合を開始",
        ))
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GlobalReversiStats>(Arc::new(Mutex::new(HashMap::new())));
        data.insert::<GlobalQuantumGomokuStats>(Arc::new(Mutex::new(HashMap::new())));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
