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
