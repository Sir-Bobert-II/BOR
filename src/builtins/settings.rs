use std::path::PathBuf;
use std::sync::Mutex;

use crate::config::{self, GuildSettings, WarnBehavior};
use crate::CONFIG;
use lazy_static::lazy_static;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{command::CommandOptionType, GuildId, PartialChannel},
        Permissions,
    },
};

lazy_static! {
    static ref SETTINGS_PATH: PathBuf = CONFIG.resources.guild_settings.clone();
    static ref SETTINGS: Mutex<GuildSettings> = Mutex::new({
        let path = CONFIG.resources.guild_settings.clone();
        if !path.exists()
        {
            GuildSettings::new().save(path).unwrap().clone()
        }
        else
        {
            GuildSettings::load(path).unwrap()
        }
    });
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("settings")
        .description("Modify settings")
        .dm_permission(false)
        .default_member_permissions(Permissions::MANAGE_GUILD)
        .create_option(|option| {
            option
                .name("set_log")
                .kind(CommandOptionType::SubCommand)
                .description("Set the channel for logging")
                .create_sub_option(|opt| {
                    opt.name("channel")
                        .description("The channel to add")
                        .kind(CommandOptionType::Channel)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("remove_log")
                .kind(CommandOptionType::SubCommand)
                .description("Stop using the current logging channel")
        })
        .create_option(|option| {
            option
                .name("set_warn_behavior")
                .kind(CommandOptionType::SubCommand)
                .description("Set the action when a specified number of warnings is met")
                .create_sub_option(|opt| {
                    opt.name("behavior")
                        .description("Nothing, Ban, Kick, Timeout")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|opt| {
                    opt.name("max")
                        .description(
                            "The number of warnings needed to take action (e.g. 3, 255) 255 Max",
                        )
                        .kind(CommandOptionType::Integer)
                        .required(true)
                })
        })
}

pub fn set_warning_behavior(gid: &GuildId, w: WarnBehavior) -> String
{
    let (_, x) = SETTINGS.lock().unwrap().has_guild(gid);

    if let Some(x) = x
    {
        SETTINGS.lock().unwrap().guilds[x]
            .settings
            .set_warning_behavior(w);
    }
    else
    {
        let s = config::Settings::new().set_warning_behavior(w).to_owned();

        // Make a guild with the settings
        SETTINGS.lock().unwrap().add_guild(*gid, s);
    }

    "".to_string()
}

pub struct Log {}

impl Log
{
    pub fn set_log(c: PartialChannel, gid: &GuildId) -> String
    {
        let cs = c.clone().name.unwrap();
        let (_, x) = SETTINGS.lock().unwrap().has_guild(gid);

        if let Some(x) = x
        {
            SETTINGS.lock().unwrap().guilds[x]
                .settings
                .set_log_channel(c);
        }
        else
        {
            let s = config::Settings::new().set_log_channel(c).to_owned();

            // Make a guild with the settings
            SETTINGS.lock().unwrap().add_guild(*gid, s);
        }

        SETTINGS
            .lock()
            .unwrap()
            .save(SETTINGS_PATH.to_path_buf())
            .unwrap();
        format!("Set log channel to {cs}")
    }

    pub fn remove_log(gid: &GuildId) -> String
    {
        let (_, x) = SETTINGS.lock().unwrap().has_guild(gid);

        if let Some(x) = x
        {
            SETTINGS.lock().unwrap().guilds[x].settings.log_channel = None;
        }
        else
        {
            let mut s = config::Settings::new();
            s.log_channel = None;

            // Make a guild with the settings
            SETTINGS.lock().unwrap().add_guild(*gid, s);
        }
        SETTINGS
            .lock()
            .unwrap()
            .save(SETTINGS_PATH.to_path_buf())
            .unwrap();
        "Removed log channel".to_string()
    }
}
