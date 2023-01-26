use crate::CONFIG;
use leb_warn::warnings::*;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{command::CommandOptionType, GuildId},
        user::User,
        Permissions,
    },
};

use std::{path::PathBuf, sync::Mutex};
// We use lazy statics that get initialized on first reference (at runtime).
// We wrap the warning in a mutex for safe mutability
lazy_static::lazy_static! {
    static ref WARNINGS_FILE: PathBuf = CONFIG.resources.warnings.clone();
    static ref WARNINGS:  Mutex<Warnings> = {
        if !WARNINGS_FILE.exists()
        {
            let warnings = Warnings::new();
            warnings.save(WARNINGS_FILE.to_path_buf()).unwrap();
            Mutex::new(warnings)
        }
        else
        {
            Mutex::new(Warnings::load(&WARNINGS_FILE).unwrap())
        }
    };
}

pub fn warn(gid: &GuildId, user: User, reason: String) -> String
{
    let uname = user.name.clone();
    WARNINGS
        .lock()
        .unwrap()
        .add_warning(&gid, user, reason.clone());
    // Save the changes
    WARNINGS
        .lock()
        .unwrap()
        .save((&WARNINGS_FILE).to_path_buf())
        .unwrap();
    format!("Warned {uname} for {reason}.")
}

pub fn remove_warns(gid: &GuildId, user: User) -> String
{
    // Search for where the guild we're looking for is
    let guild_pos = match WARNINGS
        .lock()
        .unwrap()
        .guilds
        .clone()
        .iter()
        .position(|g| g.id.to_string() == gid.to_string())
    {
        Some(x) => x,
        None => return "There are no warnings to remove".to_string(),
    };
    // Get our guild
    let guild_warnings = WARNINGS.lock().unwrap().guilds[guild_pos].clone();
    // Search for where our user is
    let user_pos = match guild_warnings.users.iter().position(|u| u.user == user)
    {
        Some(x) => x,
        None => return "There are no warnings to remove".to_string(),
    };
    // Set the user's warnings to an empty vec
    WARNINGS.lock().unwrap().guilds[guild_pos].users[user_pos].warnings = Vec::new();
    // Set the warnings count to zero
    WARNINGS.lock().unwrap().guilds[guild_pos].users[user_pos].warning_count = 0;
    // Save the changes
    WARNINGS
        .lock()
        .unwrap()
        .save((&WARNINGS_FILE).to_path_buf())
        .unwrap();
    format!("Removed all warnings for user {}", user.name)
}

pub fn get_warns(gid: &GuildId, user: User) -> String
{
    let uname = user.name.clone();
    if let Some(warnings) = WARNINGS.lock().unwrap().get_warnings(*gid, user).clone()
    {
        warnings.to_string()
    }
    else
    {
        format!("User {} has no warnings on record.", uname)
    }
}