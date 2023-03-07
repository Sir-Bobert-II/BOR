mod builtins;
mod commands;
mod config;
mod filtering;

mod data;

extern crate bor_conversions as conversions;
extern crate bor_define as define;
extern crate bor_warn as warn;
extern crate bor_wiki as wiki;

use config::{Config, RestrictedWords};
use lazy_static::lazy_static;
use log::{error, info};
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        gateway::Ready,
        prelude::{command::Command, *},
    },
    prelude::*,
};
use std::path::PathBuf;

const CONFIG_FILE: &str = "/etc/bor/config.toml";
const NAME: &str = "bot_of_retribution";

lazy_static! {
    static ref LOGFILE: PathBuf = {
        PathBuf::from("/tmp").join(NAME).join(format!(
            "logfile_{}.log",
            chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
        ))
    };
    static ref CONFIG: config::Config = Config::from(CONFIG_FILE.into()).unwrap();
    static ref RESTRICTED_WORDS: config::RestrictedWords =
        RestrictedWords::from(CONFIG.resources.restricted_words.clone()).unwrap();
    static ref GUILD_DATA: Mutex<data::UsageData> = Mutex::new({
        match data::UsageData::load(CONFIG.resources.analytics.clone())
        {
            Some(x) => x,
            None => UsageData::default()
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>
{
    env_logger::init();
    info!("Initialized Logger");

    let token = &CONFIG.secrets.token;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await
    {
        error!("Error: {:?}", why);
    }
    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction)
    {
        if let Interaction::ApplicationCommand(command) = interaction
        {
            commands::run(ctx, command).await;
        }
    }

    // async fn message(&self, context: Context, msg: Message)
    // {
    //     // Check for restricted words and remove them
    //     if !msg.is_private()
    //         && filtering::is_restricted(msg.content.clone(), &RESTRICTED_WORDS.words)
    //     {
    //         if let Err(why) = msg.delete(&context.http).await
    //         {
    //             error!("Error removing message: {:?}", why);
    //         }
    //         let response = MessageBuilder::new()
    //             .mention(&msg.author)
    //             .push("Used a restricted word!")
    //             .build();
    //         if let Err(why) = msg.channel_id.say(&context.http, &response).await
    //         {
    //             error!("Error sending message: {:?}", why);
    //         }
    //     }
    // }

    async fn ready(&self, context: Context, ready: Ready)
    {
        info!("{} is connected!", ready.user.name);
        Command::set_global_application_commands(&context.http, |commands| {
            commands
                .create_application_command(|command| builtins::moderation::register(command))
                .create_application_command(|command| builtins::meta::register(command))
                .create_application_command(|command| builtins::settings::register(command))
                .create_application_command(|command| builtins::random::register(command))
                .create_application_command(|command| conversions::register(command))
                .create_application_command(|command| wiki::register(command))
                .create_application_command(|command| define::register(command))
                .create_application_command(|command| quote::register(command))
                .create_application_command(|command| image::register(command))
        })
        .await
        .unwrap();
        info!("Commands Initialized")
    }
}
