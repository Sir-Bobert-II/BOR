use serenity::{
    model::prelude::{GuildId, Member, UserId},
    prelude::Context,
};

/// Get a member from a user id
pub async fn member_from_id(context: &Context, gid: GuildId, id: UserId) -> Member
{
    gid.member(&context.http, id).await.unwrap()
}

pub mod kick
{
    use log::error;
    use serenity::{
        builder::CreateApplicationCommand,
        model::{
            prelude::{command::CommandOptionType, GuildId},
            user::User,
            Permissions,
        },
        prelude::Context,
    };

    use super::member_from_id;

    /// Kick a user from a guild
    pub async fn run(context: &Context, gid: &GuildId, user: &User, reason: String) -> String
    {
        let member = member_from_id(&context, *gid, user.id).await;

        match member.kick_with_reason(&context.http, &reason).await
        {
            Ok(_) => format!("Kicked '{}'", user.name),
            Err(x) =>
            {
                error!("Error kicking guild member: {:?}", x);
                format!("Error kicking guild member: {:?}", x)
            }
        }
    }

    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("exile")
            .description("Kick a member from this guild")
            .dm_permission(false)
            .default_member_permissions(Permissions::KICK_MEMBERS)
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user to kick")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("reason")
                    .description("The reason why you're kicking this user")
                    .kind(CommandOptionType::String)
                    .required(false)
            })
    }
}

pub mod ban
{
    use log::error;
    use serenity::{
        builder::CreateApplicationCommand,
        model::{
            prelude::{command::CommandOptionType, GuildId, User},
            Permissions,
        },
        prelude::Context,
    };

    use super::member_from_id;
    /// Ban a user from a guild
    pub async fn run(
        context: &Context,
        gid: &GuildId,
        user: &User,
        reason: String,
        dmd: u8,
    ) -> String
    {
        let member = member_from_id(&context, *gid, user.id).await;

        match member.ban_with_reason(&context.http, dmd, reason).await
        {
            Ok(_) => format!("Banned '{}'", user.name),
            Err(x) =>
            {
                error!("Error banning guild member: {:?}", x);
                format!("Error banning guild member: {x}")
            }
        }
    }

    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("banish")
            .description("Ban a member from this guild")
            .dm_permission(false)
            .default_member_permissions(Permissions::BAN_MEMBERS)
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user to ban")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("reason")
                    .description("The reason why you're banning this user")
                    .kind(CommandOptionType::String)
                    .required(false)
            })
            .create_option(|option| {
                option
                    .name("days")
                    .description("The number of days of messages to delete (Max: 7)")
                    .kind(CommandOptionType::Integer)
                    .required(false)
            })
    }
}

pub mod warn
{
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
        static ref WARNINGS_FILE: PathBuf = CONFIG.resources.warnings_file.clone();
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

    pub fn register_warn(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("warn")
            .description("Give a member a warning")
            .dm_permission(false)
            .default_member_permissions(Permissions::BAN_MEMBERS)
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user to warn")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("reason")
                    .description("The reason why you're warning this user")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }

    pub fn register_get_warns(
        command: &mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand
    {
        command
            .name("get_warnings")
            .description("Get all the warnings for a member")
            .dm_permission(false)
            .create_option(|option| {
                option
                    .name("user")
                    .description("The user to get warnings for")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
    }

    pub fn register_remove_warns(
        command: &mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand
    {
        command
            .name("remove_warns")
            .description("Remove all warnings for a member")
            .dm_permission(false)
            .default_member_permissions(Permissions::BAN_MEMBERS)
            .create_option(|option| {
                option
                    .name("user")
                    .description("The remove warnings from")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
    }
}
