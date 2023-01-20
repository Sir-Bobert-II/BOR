mod builtins;
mod commands;
mod config;
mod filtering;

use config::{Config, RestrictedWords};
use lazy_static::lazy_static;

use env_logger;
use log::{error, info};
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        channel::Message,
        gateway::Ready,
        prelude::{command::Command, *},
    },
    prelude::*,
    utils::MessageBuilder,
};
use std::path::PathBuf;
const CONFIG_FILE: &str = "/etc/leb/config.toml";
const NAME: &str = "law_enforcement_bot";

lazy_static! {
    static ref LOGFILE: PathBuf = {
        PathBuf::from("/tmp").join(NAME).join(format!(
            "logfile_{}.log",
            chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
        ))
    };
    static ref CONFIG: config::Config = Config::from(CONFIG_FILE.into()).unwrap();
    static ref RESTRICTED_WORDS: config::RestrictedWords = {
        let restricted_words =
            RestrictedWords::from(CONFIG.resources.restricted_words.clone()).unwrap();
        restricted_words
    };
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

    let mut client = Client::builder(&token, intents)
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

    async fn message(&self, context: Context, msg: Message)
    {
        // Check for restricted words and remove them
        if !msg.is_private() && filtering::is_restricted(msg.content.clone(), &RESTRICTED_WORDS.words)
        {
            if let Err(why) = msg.delete(&context.http).await
            {
                error!("Error removing message: {:?}", why);
            }
            let response = MessageBuilder::new()
                .mention(&msg.author)
                .push("Used a restricted word!")
                .build();
            if let Err(why) = msg.channel_id.say(&context.http, &response).await
            {
                error!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, context: Context, ready: Ready)
    {
        info!("{} is connected!", ready.user.name);
        Command::set_global_application_commands(&context.http, |commands| {
            commands
                .create_application_command(|command| builtins::users::kick::register(command))
                .create_application_command(|command| builtins::users::ban::register(command))
                .create_application_command(|command| builtins::users::warn::register_warn(command))
                .create_application_command(|command| builtins::users::warn::register_get_warns(command))
                .create_application_command(|command| builtins::users::warn::register_remove_warns(command))
                .create_application_command(|command| builtins::users::timeout::register_timeout(command))
                .create_application_command(|command| builtins::users::timeout::register_realease(command))
                .create_application_command(|command| builtins::meta::register(command))
                .create_application_command(|command| builtins::settings::register(command))
        })  
        .await
        .unwrap();
        info!("Commands Initialized")
    }
}
