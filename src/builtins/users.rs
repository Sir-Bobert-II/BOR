use log::error;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{command::CommandOptionType, GuildId, UserId},
        Permissions,
    },
    prelude::Context,
};

pub async fn exile(context: &Context, gid: GuildId, id: UserId, reason: String) -> String
{
    let member = match gid.member(&context.http, id).await
    {
        Ok(x) => x,
        Err(x) =>
        {
            error!("Error getting guild member: {:?}", x);
            return "Error getting guild member: Couldn't kick member".to_string();
        }
    };

    match member.kick_with_reason(&context.http, &reason).await
    {
        Ok(_) => "Successfully exiled member".to_string(),
        Err(x) =>
        {
            error!("Error kicking guild member: {:?}", x);
            "Error kicking guild member: Couldn't kick member".to_string()
        }
    }
}

pub fn register_exile(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("exile")
        .description("Exile a member from this guild")
        .dm_permission(false)
        .default_member_permissions(Permissions::KICK_MEMBERS)
        .create_option(|option| {
            option
                .name("id")
                .description("The user to lookup")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("Reason")
                .description("The reason why you're exiling this user")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
