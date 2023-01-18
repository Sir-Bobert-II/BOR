use log::error;
    use serenity::{
        builder::CreateApplicationCommand,
        model::{
            prelude::{command::CommandOptionType, GuildId, User},
            Permissions,
        },
        prelude::Context,
    };

/// Get a member from a user id
pub fn member_from_id(context: &Context, gid:GuildId, id: UserId) -> Result<String, String>
{
    match gid.member(&context.http, id).await
    {
        Ok(x) => Ok(x),
        Err(x) =>
        {
            error!("Error getting guild member: {:?}", x);
            Err("Error getting guild member: Couldn't kick member".to_string());
        }
    }
}

pub mod kick
{
    use log::error;
    use serenity::{
        builder::CreateApplicationCommand,
        model::{
            prelude::{command::CommandOptionType, GuildId, UserId},
            Permissions,
        },
        prelude::Context,
    };

    /// Kick a user from a guild
    pub async fn run(context: &Context, gid: GuildId, user: User, reason: String) -> String
    {
        let member =match gid.member(&context, gid, user.id).await
        {
            Ok(x) => Ok(x),
            Err(x) =>
            {
                error!("{x}");
                return Err(x)
            }
        }
    

        match member.kick_with_reason(&context.http, &reason).await
        {
            Ok(_) => format!("Kicked '{}'", user.name)
            Err(x) =>
            {
                error!("Error kicking guild member: {:?}", x);
                "Error kicking guild member: Couldn't kick member".to_string()
            }
        }
    }

    pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("Kick")
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
                    .name("Reason")
                    .description("The reason why you're kicking this user")
                    .kind(CommandOptionType::String)
                    .required(false)
            })
    }
}

pub mod Ban {
    use log::error;
    use serenity::{
        builder::CreateApplicationCommand,
        model::{
            prelude::{command::CommandOptionType, GuildId, User},
            Permissions,
        },
        prelude::Context,
    };
    /// Ban a user from a guild
    pub async fn run(context: &Context, gid: GuildId, user: User, reason: String, dmd:u8) -> String
    {
        let member =match gid.member(&context, gid, user.id).await
        {
            Ok(x) => Ok(x),
            Err(x) =>
            {
                error!("{x}");
                return Err(x)
            }
        }

        match gid.member(&context, gid, user.id).await
        {
            Ok(_) => format!("Banned '{}'", user.name)
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
            .name("ban")
            .description("Ban a member from this guild")
            .dm_permission(false)
            .default_member_permissions(Permissions::BAN_MEMBERS)
            .create_option(|option| {
                option
                    .name("User")
                    .description("The user to ban")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("Reason")
                    .description("The reason why you're banning this user")
                    .kind(CommandOptionType::String)
                    .required(false)
            })
            .create_option(|option|
            {
                option
                    .name("Delete Messages")
                    .description("The number of days of messages to delete")
                    .kind(CommandOptionType:Number)
                    .required(false)
            })
    }
}

pub mod Timeout
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

    /// Timeout a user in a guild
    pub fn timeout(context: &Context, gid: GuildId, user: User, until: Timestamp) -> String
    {
        let member = match gid.member(&context, gid, user.id).await
        {
            Ok(x) => x,
            Err(x) =>
            {
                error!("{x}");
                return x
            }
        }

        match member.disable_communication_until_datetime(&context.http, until)
        {
            Ok(_) => format!("Timed out '{}'", user.name)
            Err(x) =>
            {
                error!("Error timing out guild member: {:?}", x);
                format!("Error timing out guild member: {x}")
            }
        }

    }

    pub fn timein(context: &Context, gid: GuildId, user: User) -> String
    {
        let member = match gid.member(&context, gid, user.id).await
        {
            Ok(x) => Ok(x),
            Err(x) =>
            {
                error!("{x}");
                return Err(x)
            }
        }

        match member.enable_communication(&context.http)
        {
            Ok(_) => format!("timed in '{}'", user.name)
            Err(x) =>
            {
                error!("Error timing in guild member: {:?}", x);
                format!("Error timing in guild member: {x}")
            }
        }
    }


    pub fn register_timeout(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("timeout")
            .description("Timeout a user")
            .dm_permission(false)
            .default_member_permissions(Permissions::MODERATE_MEMBERS)
            .create_option(|option| {
                option
                    .name("User")
                    .description("The user to timeout")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("Time")
                    .description("How long to timeout a user for")
                    .kind(CommandOptionType::Timestamp)
                    .required(true)
            })
    }

    pub fn register_timein(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
    {
        command
            .name("timein")
            .description("End a User's timeout")
            .dm_permission(false)
            .default_member_permissions(Permissions::MODERATE_MEMBERS)
            .create_option(|option| {
                option
                    .name("User")
                    .description("The user to timein")
                    .kind(CommandOptionType::User)
                    .required(true)
            })
    }
}