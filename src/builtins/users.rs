    use serenity::{
        model::{
            prelude::{ GuildId, Member, UserId},
        },
        prelude::Context,
    };

/// Get a member from a user id
pub async fn member_from_id(context: &Context, gid:GuildId, id: UserId) -> Member
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
            Permissions, user::User,
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

pub mod ban {
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
    pub async fn run(context: &Context, gid: &GuildId, user: &User, reason: String, dmd:u8) -> String
    {
        let member = member_from_id(&context, *gid, user.id).await;

        match member.ban_with_reason(&context.http,dmd, reason ).await
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
            .create_option(|option|
            {
                option
                    .name("days")
                    .description("The number of days of messages to delete (Max: 7)")
                    .kind(CommandOptionType::Integer)
                    .required(false)
            })
    }
}

