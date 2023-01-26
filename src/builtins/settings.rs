use std::sync::Mutex;

use crate::config::{self, GuildSettings};
use crate::CONFIG;
use lazy_static::lazy_static;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::{GuildId, PartialChannel};
use serenity::model::Permissions;

lazy_static! {
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
}

// pub fn set_warning_behavior()

pub struct Log {}

impl Log
{
    pub fn set_log(c: PartialChannel, gid: &GuildId) -> String
    {
        let cs = c.clone().name.unwrap();
        let (_, x) = SETTINGS.lock().unwrap().has_guild(&gid);

        // If the guild exits
        if x.is_some()
        {
            SETTINGS.lock().unwrap().guilds[x.unwrap()]
                .settings
                .set_log_channel(c);
            format!("Set log channel to {}", cs)
        }
        // if it doesnt
        else
        {
            let s = config::Settings::new().set_log_channel(c).to_owned();

            // Make a guild with the settings
            SETTINGS.lock().unwrap().add_guild(*gid, s);
            format!("Set log channel to {}", cs)
        }
    }

    pub fn remove_log(gid: &GuildId)
    {
        let (_, x) = SETTINGS.lock().unwrap().has_guild(&gid);

        // If the guild exits
        if x.is_some()
        {
            SETTINGS.lock().unwrap().guilds[x.unwrap()]
                .settings
                .log_channel = None;
        }
        // if it doesnt
        else
        {
            let mut s = config::Settings::new().to_owned();
            s.log_channel = None;

            // Make a guild with the settings
            SETTINGS.lock().unwrap().add_guild(*gid, s);
        }
    }
}
