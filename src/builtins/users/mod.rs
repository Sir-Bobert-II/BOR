use serenity::{
    model::prelude::{GuildId, Member, UserId},
    prelude::Context,
};

/// Get a member from a user id
pub async fn member_from_id(context: &Context, gid: GuildId, id: UserId) -> Member
{
    gid.member(&context.http, id).await.unwrap()
}

pub mod kick;
pub mod ban;
pub mod warn;
pub mod timeout;