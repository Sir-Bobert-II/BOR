use log::error;
use serenity::{
    model::prelude::interaction::{
        application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
        InteractionResponseType,
    },
    prelude::Context,
};

use crate::builtins::users;


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
                users::kick::run(&context, guild_id, user, reason).await
            }
            else
            {
                "Error: Inproper value provided".to_string()
            }
        },

        "ban" => {
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
                users::ban::run(&context, guild_id, user, reason).await
            }
            else
            {
                "Error: Inproper value provided".to_string()
            }
        }

        "timeout" => {
            let user;
            let timestamp;
            for option in command.data.options
            {
                let opt = option.resolved.as_ref().unwrap();
                
                match &opt.name
                {
                    "User" => {
                        if let CommandDataOptionValue::User(u, _) = opt
                        {
                            user = Some(u);
                        }
                    },

                    "Time" => {
                        if let CommandDataOptionValue::Timestamp(t, _) = opt
                        {
                            timestamp = Some(t);
                        }
                    }

                    _=> unreachable!();
                    
                }
            }
            let guild_id = command.guild_id.unwrap();

            if let (Some(user), Some(time)) = (user, timestamp) 
            {
                users::timeout::timeout(&context, guild_id, user, time)
            }
            else
            {
                "Error: Internal command parameter problem".to_string()
            }

            
        }

        "timein" => {
            let user;
            for option in command.data.options
            {
                let opt = option.resolved.as_ref().unwrap();
                
                match &opt.name
                {
                    "User" => {
                        if let CommandDataOptionValue::User(u, _) = opt
                        {
                            user = Some(u);
                        }
                    },

                    _=> unreachable!();
                    
                }
            }
            let guild_id = command.guild_id.unwrap();

            if let (Some(user), Some(time)) = (user, timestamp) 
            {
                users::timeout::timein(&context, guild_id, user);
            }
            else
            {
                "Error: Internal command parameter problem".to_string()
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
