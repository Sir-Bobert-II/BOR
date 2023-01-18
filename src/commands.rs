use log::error;
use serenity::{
    model::prelude::interaction::{
        application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
        InteractionResponseType,
    },
    prelude::Context,
};

use crate::builtins::users::kick;

pub async fn run(context: Context, command: ApplicationCommandInteraction)
{
    let command_name = command.data.name.as_str();
    let content = match command_name
    {
        "kick" =>
        {
            let option_user = command
                .data
                .options
                .get(0)
                .expect("Expected user option")
                .resolved
                .as_ref()
                .expect("Expected user object");
            let mut reason: String = "No reason provided.".to_string();
            if command.data.options.len() > 1
            {
                let option_reason = command
                    .data
                    .options
                    .get(1)
                    .expect("Expected user option")
                    .resolved
                    .as_ref()
                    .expect("Expected user object");

                if let CommandDataOptionValue::String(x) = option_reason
                {
                    reason = x.clone();
                }
            }

            let guild_id = command.guild_id.unwrap();

            if let CommandDataOptionValue::User(user, _member) = option_user
            {
                kick::run(&context, guild_id, user.id, reason).await
            }
            else
            {
                "Error: Inproper value provided".to_string()
            }
        }

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
