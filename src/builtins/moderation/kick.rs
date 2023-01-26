use log::error;
use serenity::{
    model::{prelude::GuildId, user::User},
    prelude::Context,
};

use super::member_from_id;

/// Kick a user from a guild
pub async fn run(context: &Context, gid: &GuildId, user: &User, reason: String) -> String
{
    let member = member_from_id(context, *gid, user.id).await;

    match member.kick_with_reason(&context.http, &reason).await
    {
        Ok(_) => format!("Kicked '{}'", user.name),
        Err(x) =>
        {
            error!("Error kicking guild member: {:?}", x);
            format!("Error kicking guild member: {x:?}")
        }
    }
}
