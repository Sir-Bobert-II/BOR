mod builtins;
mod commands;
mod config;
mod error;
mod filtering;

use config::{Config, RestrictedWords};
use error::Error;
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
async fn main() -> Result<(), Error>
{
    // if !LOGFILE.parent().unwrap().exists()
    // {
    //     create_dir_all(LOGFILE.parent().unwrap()).unwrap();
    // }

    // wd_log::output_to_file(LOGFILE.to_owned()).unwrap();

    // env_logger::init_from_env(
    //     env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    // );
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
        if filtering::is_restricted(msg.content.clone(), &RESTRICTED_WORDS.words)
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

        if msg.content == "!ping"
        {
            let channel = match msg.channel_id.to_channel(&context).await
            {
                Ok(channel) => channel,
                Err(why) =>
                {
                    error!("Error getting channel: {:?}", why);

                    return;
                }
            };

            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the 'ping' command in the ")
                .mention(&channel)
                .push(" channel")
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
                .create_application_command(|command| builtins::meta::register(command))
        })
        .await
        .unwrap();
        info!("Commands stetp")
    }
}
