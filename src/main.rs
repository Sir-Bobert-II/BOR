mod builtins;
mod commands;
mod config;
mod data;
mod filtering;

extern crate bor_conversions as conversions;
extern crate bor_define as define;
extern crate bor_warn as warn;
extern crate bor_wiki as wiki;

use chrono::{Duration, Utc};
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
use structopt::StructOpt;
use tokio::spawn;

use std::path::PathBuf;
use tokio_schedule::Job;

const CONFIG_FILE: &str = "/etc/bor/config.toml";
const NAME: &str = "bot_of_retribution";

lazy_static! {
    static ref LOGFILE: PathBuf = {
        PathBuf::from("/tmp").join(NAME).join(format!(
            "logfile_{}.log",
            chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
        ))
    };
    static ref CONFIG: config::Config = Config::from(config_file()).unwrap();
    static ref RESTRICTED_WORDS: config::RestrictedWords =
        RestrictedWords::from(CONFIG.resources.restricted_words.clone()).unwrap();
    pub static ref DATA: Mutex<data::UsageData> = Mutex::new({
        match data::UsageData::load(CONFIG.resources.analytics.clone()) {
            Some(x) => x,
            None => data::UsageData::default(),
        }
    });
    static ref CMD_OPTIONS: Options = Options::from_args();
}

fn config_file() -> PathBuf
{
    match &*CMD_OPTIONS {
        Options::Daemon {
            config_file: config,
        } => config.clone(),
        _ => CONFIG_FILE.into(),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "bor",
    about = "Bot of Retribution is an open-source, self-hostable, dicord bot"
)]
enum Options
{
    /// Run the disord bot
    Daemon
    {
        /// Use this path instead of the default config file path
        #[structopt(short = "c", long = "config", default_value = CONFIG_FILE)]
        config_file: PathBuf,
    },

    /// Read/Write configurations and other stored data
    Info(InfoSubOptions),
}

#[derive(Debug, StructOpt)]
enum InfoSubOptions
{
    /// Set the discord token
    SetToken
    {
        /// Discord token
        token: String,
    },

    /// List the guilds that have used commands in the past month
    ListGuilds,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>
{
    match &*CMD_OPTIONS {
        Options::Info(opt) => {
            match opt {
                InfoSubOptions::ListGuilds => {
                    // Print all the guilds that have used commands
                    let data = DATA.lock().await;
                    for guild in data.gdata.iter() {
                        let id = &guild.1.gid;
                        let name = &guild.1.gname;
                        println!("{name} ({id})");
                    }
                }

                InfoSubOptions::SetToken { token } => {
                    let mut conf = CONFIG.clone();
                    conf.set_token(token.clone());
                    match conf.save(config_file()) {
                        Err(e) => eprintln!("Couldn't save config file changes: {e}"),
                        _ => {
                            println!("Set new token. Restart any bots that use this configuration.")
                        }
                    };
                }
            }

            // Stop execution here
            std::process::exit(0);
        }
        _ => (),
    }

    env_logger::init();
    info!("Initialized Logger");

    let data_management = tokio_schedule::every(1)
        .day()
        .in_timezone(&Utc)
        .perform(data::mangage_data);
    spawn(data_management);

    let token = &CONFIG.secrets.token;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Error: {:?}", why);
    }

    Ok(())
}

#[cfg(test)]
mod tests
{
    use crate::data::{GuildData, UsageData};
    use std::io::Write;

    use super::*;
    #[tokio::test]
    async fn test_build_graphs_logic()
    {
        use poloto::{build, num::timestamp};
        use std::collections::HashMap;
        let data = UsageData {
            cdata: HashMap::new(),
            udata: HashMap::new(),
            gdata: HashMap::from([
                (
                    GuildId(122),
                    GuildData {
                        gid: GuildId(122),
                        gname: "PCMR".to_string(),
                        requests: vec![
                            (5600, Utc::now() - Duration::hours(5)),
                            (1000, Utc::now() - Duration::hours(4)),
                            (6000, Utc::now() - Duration::hours(3)),
                            (60000, Utc::now() - Duration::hours(2)),
                            (6000, Utc::now()),
                        ],
                    },
                ),
                (
                    GuildId(23),
                    GuildData {
                        gid: GuildId(23),
                        gname: "PCMR".to_string(),
                        requests: vec![
                            (5600, Utc::now() - Duration::hours(5)),
                            (1000, Utc::now() - Duration::hours(4)),
                            (6000, Utc::now() - Duration::hours(3)),
                            (60000, Utc::now() - Duration::hours(2)),
                            (6000, Utc::now()),
                        ],
                    },
                ),
            ]),
        };


        for (gid, gdat) in data.gdata.iter() {
            let data = gdat
                .requests
                .iter()
                .map(|(x, y)| (timestamp::UnixTime::from(*y), *x as f64));

            let scratch = crate::CONFIG.resources.scratch.clone();
            if !scratch.exists() {
                std::fs::create_dir_all(&scratch).unwrap();
            }
            let plot = poloto::data(poloto::plots!(build::plot("").histogram(data)))
                .build_and_label((
                    format!("Command Calls for '{} ({})'", gdat.gname, gid),
                    "Time",
                    "Command Calls",
                ))
                .append_to(poloto::header().dark_theme())
                .render_string()
                .unwrap();
            let mut f =
                std::fs::File::create(scratch.join(format!("{}.svg", gid.to_string()))).unwrap();
            f.write_all(plot.as_bytes()).unwrap();
        }

        assert!(false)
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction)
    {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Running command '{}'", command.data.name);
            commands::run(ctx, command).await;
        }
    }

    // async fn message(&self, context: Context, msg: Message)
    // {
    //     // Check for restricted words and remove them
    //     if !msg.is_private()
    //         && filtering::is_restricted(msg.content.clone(),
    // &RESTRICTED_WORDS.words)     {
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
        info!("Registering commands...");
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
        .expect("Unable to register commands.");
        info!("Commands registered.")
    }
}
