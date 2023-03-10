use std::{path::PathBuf, sync::Mutex};

use crate::{config::WarnBehavior, CONFIG};
use log::info;
use serenity::{
    model::{prelude::GuildId, user::User},
    prelude::Context,
};

// We use lazy statics that get initialized on first reference (at runtime).
// We wrap the warning in a mutex for safe mutability

lazy_static::lazy_static! {
    static ref WARNINGS_FILE: PathBuf = CONFIG.resources.warnings.clone();
    static ref WARNINGS:  Mutex<bor_warn::warnings::Warnings> = {
        if !WARNINGS_FILE.exists()
        {
            let warnings = bor_warn::warnings::Warnings::new();
            warnings.save(WARNINGS_FILE.to_path_buf()).unwrap();
            Mutex::new(warnings)
        }
        else
        {
            Mutex::new(bor_warn::warnings::Warnings::load(&WARNINGS_FILE).unwrap())
        }
    };
}

use crate::builtins::settings::SETTINGS as GUILD_SETTINGS;

pub async fn warn(context: &Context, gid: &GuildId, user: User, reason: String) -> String
{
    let uname = user.name.clone();
    let warning_behavior = {
        if let (gs, Some(i)) = GUILD_SETTINGS.lock().unwrap().has_guild(gid) {
            gs.guilds[i].settings.warning_behavior
        } else {
            WarnBehavior::Nothing
        }
    };

    let count = WARNINGS
        .lock()
        .unwrap()
        .add_warning(gid, &user, reason.clone())
        .count_warnings(gid, &user);

    match warning_behavior {
        WarnBehavior::Ban(cap) => {
            // Ban the user if we're at the warning limit
            if count >= cap.into() {
                super::ban::run(
                    context,
                    gid,
                    &user,
                    format!("Banned for accumulating {count} warnings."),
                    0,
                )
                .await;
            }
        }
        WarnBehavior::Nothing => (),
        WarnBehavior::Kick(cap) => {
            // Ban the user if we're at the warning limit
            if count >= cap.into() {
                super::kick::run(
                    context,
                    gid,
                    &user,
                    format!("Banned for accumulating {count} warnings."),
                )
                .await;
            }
        }
        WarnBehavior::Timeout {
            warning_count,
            duration,
        } => {
            // Ban the user if we're at the warning limit
            if count >= warning_count.into() {
                super::timeout::timeout(context, gid, user, duration, true).await;
            }
        }
    }

    // Save the changes
    WARNINGS
        .lock()
        .unwrap()
        .save(WARNINGS_FILE.to_path_buf())
        .unwrap();

    let s = format!("Warned {uname} for {reason}.");
    info!("{s}");
    s
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
    let user_pos = match guild_warnings.users.iter().position(|u| u.user == user) {
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
        .save(WARNINGS_FILE.to_path_buf())
        .unwrap();
    let s = format!("Removed all warnings for user {}", user.name);

    info!("{s}");
    s
}

pub fn get_warns(gid: &GuildId, user: User) -> String
{
    let uname = user.name.clone();
    let s = if let Some(warnings) = WARNINGS.lock().unwrap().get_warnings(*gid, user) {
        warnings.to_string()
    } else {
        format!("User {uname} has no warnings on record.")
    };

    info!("{s}");
    s
}
