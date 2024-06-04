use serenity::all::{Command, Ready};
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
mod reversi;

use components::{choice_pos, push_stone};
use global_data::GlobalReversiStats;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = &interaction {
            println!("SlashCommand Used: {}", command.data.name);

            let (content, components) = match command.data.name.as_str() {
                "ping" => (commands::ping::run(&command.data.options()), None),
                "match_start" => {
                    commands::match_start::run(&command.data.options(), &command, &ctx).await
                }
                "match_end" => {
                    commands::match_end::run(&command.data.options(), &command, &ctx).await
                }

                _ => (Some("not implemented".to_string()), None),
            };

            if let Some(content) = content {
                let mut data = CreateInteractionResponseMessage::new().content(content);

                data = match components {
                    Some(components) => data.components(components),
                    None => data,
                };

                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }

        if let Interaction::Component(interaction) = &interaction {
            let custom_id = &interaction.data.custom_id;

            match custom_id.as_str() {
                "choice_alphabet" => choice_pos::alphabet(&ctx, interaction).await,
                "choice_number" => choice_pos::number(&ctx, interaction).await,
                "push_stone" => push_stone::run(&ctx, interaction).await,

                _ => (),
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("login: {}", ready.user.name);

        for i in [
            commands::ping::register(),
            commands::match_start::register(),
            commands::match_end::register(),
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
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GlobalReversiStats>(Arc::new(Mutex::new(HashMap::new())));
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
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<GlobalReversiStats>(Arc::new(Mutex::new(HashMap::new())));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
