use log::error;
use serenity::{
    model::prelude::interaction::{
        application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
        InteractionResponseType,
    },
    prelude::Context,
};

use crate::builtins::{users, meta};
pub async fn run(context: Context, command: ApplicationCommandInteraction)
{
    let command_name = command.data.name.as_str();
    let content = match command_name
    {
        "kick" =>
        {
            let mut user = None;
            let mut reason = "No reason provided.".to_string();
            for option in command.data.options.clone()
            {
                let opt = option.resolved.unwrap();

                match &*option.name
                {
                    "User" =>
                    {
                        if let CommandDataOptionValue::User(u, _) = opt
                        {
                            user = Some(u);
                        }
                    }
                    "Reason" =>
                    {
                        if let CommandDataOptionValue::String(r) = opt
                        {
                            reason = r.clone();
                        }
                    }

                    _ => unreachable!(),
                }
            }

            if user.is_none()
            {
                error!("Cannot respond to slash command: No 'User' provided");
                return;
            }
            let user = user.unwrap();

            let guild_id = command.guild_id.unwrap();

            users::kick::run(&context, &guild_id, &user, reason).await;
            format!("Kicked '{}'.", user.name)
        }

        "ban" =>
        {
            let mut user = None;
            let mut days: u8 = 0;
            let mut reason = "No reason provided.".to_string();

            for option in command.data.options.clone()
            {
                let opt = option.resolved.unwrap();

                match &*option.name
                {
                    "User" =>
                    {
                        if let CommandDataOptionValue::User(u, _) = opt
                        {
                            user = Some(u);
                        }
                    }
                    "Reason" =>
                    {
                        if let CommandDataOptionValue::String(r) = opt
                        {
                            reason = r.clone();
                        }
                    }
                    "Delete Messages" =>
                    {
                        if let CommandDataOptionValue::Integer(i) = opt
                        {
                            days = i.clamp(0, 7) as u8;
                        }
                    }

                    _ => unreachable!(),
                }
            }

            if user.is_none()
            {
                error!("Cannot respond to slash command: No 'User' provided");
                return;
            }
            let user = user.unwrap();

            let guild_id = command.guild_id.unwrap();

            users::ban::run(&context, &guild_id, &user, reason, days).await;
            format!("Banned '{}'.", user.name)
        }

        "meta" => meta::meta(),
        _ => format!("Error: Unrecognized command {}.", command_name),
    };

    if let Err(why) = command
        .create_interaction_response(&context.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content))
        })
        .await
    {
        error!("Cannot respond to slash command: {}", why);
    }
}
