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