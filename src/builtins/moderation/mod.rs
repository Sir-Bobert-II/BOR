//! Moderation functions for guilds to moderate members. Not available in DM's

use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{command::CommandOptionType, GuildId, Member, UserId},
        Permissions,
    },
    prelude::Context,
};

/// Get a member from a user id
pub async fn member_from_id(context: &Context, gid: GuildId, id: UserId) -> Member
{
    gid.member(&context.http, id).await.unwrap()
}

pub mod ban;
pub mod kick;
pub mod timeout;
pub mod warn;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("moderation")
        .description("Perform administrative action upon members")
        .dm_permission(false)
        .default_member_permissions(Permissions::BAN_MEMBERS)
        .create_option(|option| {
            option
                .name("ban")
                .kind(CommandOptionType::SubCommand)
                .description("Ban a member from this guild")
                .create_sub_option(|opt| {
                    opt.name("user")
                        .description("The user to ban")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
                .create_sub_option(|opt| {
                    opt.name("reason")
                        .description("The reason why you're banning this user")
                        .kind(CommandOptionType::String)
                        .required(false)
                })
                .create_sub_option(|opt| {
                    opt.name("days")
                        .description("The number of days of messages to delete (Max: 7)")
                        .kind(CommandOptionType::Integer)
                        .required(false)
                })
        })
        .create_option(|option| {
            option
                .name("kick")
                .kind(CommandOptionType::SubCommand)
                .description("Kick a member from this guild")
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user to kick")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("reason")
                        .description("The reason why you're kicking this user")
                        .kind(CommandOptionType::String)
                        .required(false)
                })
        })
        .create_option(|option| {
            option
                .name("get_warnings")
                .description("Get all the warnings for a member")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user to get warnings for")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("remove_warnings")
                .kind(CommandOptionType::SubCommand)
                .description("Remove all warnings for a member")
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The remove warnings from")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("timeout")
                .description("Give a member a timeout")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user to timeout")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("days")
                        .description("The number of days to timeout for")
                        .kind(CommandOptionType::Integer)
                        .required(false)
                })
                .create_sub_option(|option| {
                    option
                        .name("hours")
                        .description("The number of hours to timeout for")
                        .kind(CommandOptionType::Integer)
                        .required(false)
                })
                .create_sub_option(|option| {
                    option
                        .name("minutes")
                        .description("The number of minutes to timeout for")
                        .kind(CommandOptionType::Integer)
                        .required(false)
                })
                .create_sub_option(|option| {
                    option
                        .name("seconds")
                        .description("The number of seconds to timeout for")
                        .kind(CommandOptionType::Integer)
                        .required(false)
                })
        })
        .create_option(|option| {
            option
                .name("release")
                .description("Release a member from their timeout")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user to release")
                        .kind(CommandOptionType::User)
                        .required(true)
                })
        })
}
